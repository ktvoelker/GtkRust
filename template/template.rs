
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

enum delims = {
  start: char,
  end: char
};

pure fn default_delims() -> delims {
  ret delims({start: '$', end: '$'});
}

mod token {
  enum kw {
    kw_comment,
    kw_comma,
    kw_if,
    kw_else,
    kw_not,
    kw_exists,
    kw_enter,
    kw_for,
    kw_in,
    kw_error,
    kw_index,
    kw_timestamp
  }

  enum token {
    text(str),
    start,
    end,
    kw(str),
    name(name)
  }

  fn tokenize_cs(delims: delims, xs: [char]) -> @list<token> {
    TODO();
  }

  fn tokenize_cs_text(delims: delims, xs: [char]) -> @list<token> {
    TODO();
  }

  fn tokenize_text(delims: delims, xs: [char]) -> @list<token> {
    ret @ if (xs.len() == 0u) {
      // end of input
      nil
    } else if (xs[0u] == delims.start) {
      if (xs.len() == 1u) {
        // syntax error: input ends with the opening delimiter
        fail "Input ends with the opening delimiter"
      } else if (xs[1u] == delims.start) {
        // repeated delimiter is escape sequence for the delimiter as text
        cons(
            text(str::from_char(delims.start)),
            tokenize_text(delims, tailn(xs, 2u)))
      } else {
        // we found a starting delimiter
        cons(
            start,
            tokenize_cs(delims, tail(xs)))
      }
    } else {
      // plain text
      alt position(xs, {|c| c == delims.start }) {
        // no more starting delimiters in the input
        none {
          cons(text(str::from_chars(xs)), @nil)
        }
        // jump to the next starting delimiter
        some(pos) {
          cons(
            text(str::from_chars(slice(xs, 0u, pos))),
            tokenize_text(delims, tailn(xs, pos)))
        }
      }
    }
  }
}

fn parse(delims: delims, xs: str) -> template {
  let toks = token::tokenize_text(delims, str::chars(xs));
  TODO();
}

