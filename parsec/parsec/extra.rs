
import core::*;

fn takeWhere<T: copy>(p: fn@(T) -> bool) -> parser<T, T> {
  peek().andThen(where(p)).andThen(pass(skip()))
}

