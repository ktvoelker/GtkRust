
enum state<T> = @{
  xs: [T],
  mut pos: uint,
  mut err: option<str>
};

fn id<T: copy>(x: T) -> T { x }

impl state<T> for state<T> {

  fn apply1<U, V>(f: fn(U) -> option<V>, x: U) -> option<V> {
    alt self.err {
      none { f(x) }
      some(_) { none }
    }
  }

  fn apply<U>(f: fn(state<T>) -> option<U>) -> option<U> {
    self.apply1(f, self)
  }

  fn return<U: copy>(x: U) -> option<U> {
    self.apply1({ |x| some(x) }, x)
  }

  fn error<U>(msg: str) -> option<U> {
    alt self.err {
      none { self.err = some(msg); }
      some(_) { log(error, "A second error occurred"); }
    };
    ret none;
  }

}

type parser<T, U: copy> = fn@(state<T>) -> option<U>;

type p_parser<P: copy, T, U: copy> = fn@(P) -> parser<T, U>;

fn eof<T>(s: state<T>) -> option<()> {
  if (s.pos < s.xs.len()) {
    s.error("Expected end-of-file")
  } else {
    s.return(())
  }
}

fn skip_n<T>(n: uint) -> parser<T, ()> {
  ret fn@(s: state<T>) -> option<()> {
    if (s.pos + n > s.xs.len()) {
      s.error("Tried to skip past end of input")
    } else {
      s.pos += n;
      s.return(())
    }
  };
}

fn skip<T>() -> parser<T, ()> {
  skip_n(1u)
}

fn peek_n<T: copy>(n: uint) -> parser<T, [T]> {
  ret fn@(s: state<T>) -> option<[T]> {
    if (s.pos + n > s.xs.len()) {
      s.error("Tried to look past end of input")
    } else {
      s.return(s.xs.slice(0u, n))
    }
  };
}

fn peek<T: copy>() -> parser<T, T> {
  peek_n(1u).map({ |xs| vec::head(xs) })
}

fn take_n<T: copy>(n: uint) -> parser<T, [T]> {
  peek_n(n).andThen(pass(skip_n(n)))
}

fn take<T: copy>() -> parser<T, T> {
  take_n(1u).map({ |xs| vec::head(xs) })
}

fn where<T: copy, U: copy>(p: fn@(U) -> bool) -> p_parser<U, T, U> {
  ret fn@(x: U) -> parser<T, U> {
    if (p(x)) {
      fn@(s: state<T>) -> option<U> { s.return(x) }
    } else {
      fn@(s: state<T>) -> option<U> { s.error("Predicate check failed") }
    }
  };
}

fn ignore<T, U: copy>() -> p_parser<U, T, ()> {
  fn@(_x: U) -> parser<T, ()> {
    fn@(s: state<T>) -> option<()> {
      s.return(())
    }
  }
}

fn pass<T: copy, U: copy>(p: parser<T, ()>) -> p_parser<U, T, U> {
  fn@(val: U) -> parser<T, U> {
    fn@(s: state<T>) -> option<U> {
      p(s);
      s.return(val)
    }
  }
}

impl parser<T: copy, U: copy> for parser<T, U> {

  fn andThen<V: copy>(next: p_parser<U, T, V>) -> parser<T, V> {
    fn@(s: state<T>) -> option<V> {
      s.apply(self).map_default(none, { |x|
        s.apply(next(x))
      })
    }
  }

  fn map<V: copy>(f: fn@(x: U) -> V) -> parser<T, V> {
    self.andThen(fn@(x: U) -> parser<T, V> {
      fn@(s: state<T>) -> option<V> { s.return(f(x)) }
    })
  }

  fn maybeTryOrElse<U: copy>(backtrack: bool, other: parser<T, U>) -> parser<T, U> {
    ret fn@(s: state<T>) -> option<U> {
      let orig_pos = s.pos;
      s.apply({ |s|
        alt self(s) {
          none {
            if (backtrack || s.pos == orig_pos) {
              s.pos = orig_pos;
              s.err = none;
              other(s)
            } else {
              none
            }
          }
          some(r) {
            some(r)
          }
        }
      })
    };
  }

  fn tryParsePartial(xs: [T]) -> (state<T>, option<U>) {
    let s = state(@{xs: xs, mut pos: 0u, mut err: none});
    let r = self.andThen(pass(bind eof(_)))(s);
    ret (s, r);
  }

  fn tryParse(xs: [T]) -> option<U> {
    tuple::second(self.tryParsePartial(xs))
  }

  fn parse(xs: [T]) -> U {
    alt self.tryParsePartial(xs) {
      (s, none) {
        fail s.err.map_default("Unknown error", bind id(_));
      }
      (_, some(r)) {
        ret r;
      }
    };
  }

}

/*
fn never_parser<T, U>(_s: state<T>) -> option<U> { none }

fn never_p_parser<P, T, U: copy>(_val: P) -> parser<T, U> { bind never_parser(_) }

impl option_parser<T: copy, U: copy> for option<parser<T, U>> {

  fn andThen<V: copy>(next: p_parser<U, T, V>) -> parser<T, V> {
    self.map_default(bind never_parser(_), { |p| p.andThen(next) })
  }

  fn maybeTryOrElse<U: copy>(backtrack: bool, other: parser<T, U>) -> parser<T, U> {
    self.map_default(bind never_parser(_), { |p| p.maybeTryOrElse(backtrack, other) })
  }

}
*/

