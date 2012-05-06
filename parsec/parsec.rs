
enum state<T> = @{
  xs: [T],
  mut pos: uint
};

type parser<T, U: copy> = fn@(state<T>) -> U;

type p_parser<P: copy, T, U: copy> = fn@(P) -> parser<T, U>;

fn eof<T>(s: state<T>) -> () {
  if (s.pos < s.xs.len()) {
    log(info, s);
    fail "Expected end-of-file";
  }
  ret ();
}

impl parser<T: copy, U: copy> for parser<T, U> {

  fn ignore() -> parser<T, ()> {
    ret fn@(s: state<T>) -> () {
      self(s);
      ret ();
    };
  }

  fn pass<V: copy>() -> p_parser<V, T, V> {
    ret fn@(val: V) -> parser<T, V> {
      ret fn@(s: state<T>) -> V {
        self(s);
        ret val;
      };
    };
  }

  fn andThen<V: copy>(next: p_parser<U, T, V>) -> parser<T, V> {
    ret fn@(s: state<T>) -> V {
      ret next(self(s))(s);
    };
  }

  fn parse(xs: [T]) -> U {
    ret self.andThen(bind eof(_).pass())(state(@{xs: xs, mut pos: 0u}));
  }

}

