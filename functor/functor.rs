
iface functor<T, U, V> {
  fn fmap(f: fn(T) -> U) -> V;
}

impl<T, U> of functor<T, U, option<U>> for option<T> {
  fn fmap(f: fn(T) -> U) -> option<U> {
    ret alt self {
      none -> { none }
      some(val) -> { some(f(val)) }
    };
  }
}

impl<T, U, V> of functor<T, U, either<V, U>> for either<V, T> {
  fn fmap(f: fn(T) -> U) -> either<V, U> {
    ret alt self {
      left(err) -> { left(err) }
      right(val) -> { right(f(val)) }
    };
  }
}

