
import either::*;
import vec::{tail, tailn, position, slice};

use std;
import std::json::*;
import std::list;
import std::list::{list, cons, nil};

fn TODO() -> ! {
  fail #fmt("Not implemented");
}

type name = str;

enum template {
  timestamp,
  iter(either::either<name, (name, name)>, name, @template),
  index,
  enter(name, @template),
  cond(list<{
      invert: bool,
      exists: bool,
      val: name,
      body: @template
    }>, @template),
  error(str),
  subst(name),
  text(str)
}

fn parse(xs: str) -> template {
  ret parser::parse(lexer::default_delims(), xs);
}

