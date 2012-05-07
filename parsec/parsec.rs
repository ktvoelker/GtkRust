
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

fn skip<T>(n: uint) -> parser<T, ()> {
  ret fn@(s: state<T>) -> option<()> {
    if (s.pos + n > s.xs.len()) {
      s.error("Tried to skip past end of input")
    } else {
      s.pos += n;
      s.return(())
    }
  };
}

fn peek<T: copy>(n: uint) -> parser<T, [T]> {
  ret fn@(s: state<T>) -> option<[T]> {
    if (s.pos + n > s.xs.len()) {
      s.error("Tried to look past end of input")
    } else {
      s.return(s.xs.slice(0u, n))
    }
  };
}

fn take<T: copy>(n: uint) -> parser<T, [T]> {
  peek(n).andThen(skip(n).pass())
}

impl parser<T: copy, U: copy> for parser<T, U> {

  fn ignore() -> parser<T, ()> {
    ret fn@(s: state<T>) -> option<()> {
      s.apply(self);
      ret s.return(());
    };
  }

  fn pass<V: copy>() -> p_parser<V, T, V> {
    ret fn@(val: V) -> parser<T, V> {
      ret fn@(s: state<T>) -> option<V> {
        s.apply(self);
        ret s.return(val);
      };
    };
  }

  fn andThen<V: copy>(next: p_parser<U, T, V>) -> parser<T, V> {
    ret fn@(s: state<T>) -> option<V> {
      s.apply(self).map_default(none, { |x|
        s.apply(next(x))
      })
    };
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
    let r = self.andThen(bind eof(_).pass())(s);
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

fn never_parser<T, U>(_s: state<T>) -> option<U> { none }

fn never_p_parser<P, T, U: copy>(_val: P) -> parser<T, U> { bind never_parser(_) }

impl option_parser<T: copy, U: copy> for option<parser<T, U>> {

  fn ignore() -> parser<T, ()> {
    self.map_default(bind never_parser(_), { |p| p.ignore() })
  }

  fn pass<V: copy>() -> p_parser<V, T, V> {
    self.map_default(bind never_p_parser(_), { |p| p.pass() })
  }

  fn andThen<V: copy>(next: p_parser<U, T, V>) -> parser<T, V> {
    self.map_default(bind never_parser(_), { |p| p.andThen(next) })
  }

  fn maybeTryOrElse<U: copy>(backtrack: bool, other: parser<T, U>) -> parser<T, U> {
    self.map_default(bind never_parser(_), { |p| p.maybeTryOrElse(backtrack, other) })
  }

}

