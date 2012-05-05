
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
    kw_timestamp,
    kw_end
  }

  enum token {
    text(str),
    start,
    end,
    kw(kw),
    name(name)
  }

  fn is_word_char(c: char) -> bool {
    ret c == '_' || c == '-' || char::is_alphanumeric(c);
  }

  fn take_word(xs: [char]) -> {word: str, rest: [char]} {
    ret alt position(xs, {|c| !is_word_char(c)}) {
      none {
        {word: str::from_chars(xs), rest: []}
      }
      some(pos) {
        {word: str::from_chars(slice(xs, 0u, pos)), rest: tailn(xs, pos)}
      }
    };
  }

  fn tokenize_cs(delims: delims, xs: [char]) -> @list<token> {
    log(debug, #fmt("tokenize_cs(%s)", str::from_chars(xs)));
    ret @ if (xs.len() == 0u) {
      // error: control sequence never ends
      fail "Missing end delimiter"
    } else {
      alt position(xs, {|c| !char::is_whitespace(c)}) {
        none {
          // error: control sequence never ends (only whitespace remains)
          fail "Missing end delimiter"
        }
        some(pos) {
          // skip initial whitespace
          let xs = tailn(xs, pos);
          if (xs[0u] == delims.end && (xs.len() == 1u || xs[1u] != delims.end)) {
            // end delimiter
            cons(
                end,
                tokenize_text(delims, tail(xs)))
          } else if (xs[0] == ',') {
            // comma token
            cons(
                kw(kw_comma),
                tokenize_cs(delims, tail(xs)))
          } else if (xs[0] == '#') {
            // comment token puts us into cs_text mode
            cons(
                kw(kw_comment),
                tokenize_cs_text(delims, tail(xs)))
          } else {
            let {word, rest} = take_word(xs);
            if (word == "") {
              fail "Invalid control sequence"
            } else {
              // a word token
              let okw = alt word {
                "if" { some(kw_if) }
                "else" { some(kw_else) }
                "not" { some(kw_not) }
                "exists" { some(kw_exists) }
                "enter" { some(kw_enter) }
                "for" { some(kw_for) }
                "in" { some(kw_in) }
                "error" { some(kw_error) }
                "index" { some(kw_index) }
                "timestamp" { some(kw_timestamp) }
                "end" { some(kw_end) }
                _ { none }
              };
              alt okw {
                none {
                  cons(
                      name(word),
                      tokenize_cs(delims, rest))
                }
                some(kw_error) {
                  cons(
                      kw(kw_error),
                      tokenize_cs_text(delims, rest))
                }
                some(skw) {
                  cons(
                      kw(skw),
                      tokenize_cs(delims, rest))
                }
              }
            }
          }
        }
      }
    }
  }

  fn tokenize_cs_text(delims: delims, xs: [char]) -> @list<token> {
    log(debug, #fmt("tokenize_cs_text(%s)", str::from_chars(xs)));
    let xs_len = xs.len();
    ret @ if (xs_len == 0u) {
      // error: control sequence never ends
      fail "Missing end delimiter"
    } else {
      alt position(xs, {|c| c == delims.end}) {
        none {
          // error: control sequence never ends
          fail "Missing end delimiter"
        }
        some(pos) {
          if (pos + 1u < xs_len && xs[pos + 1u] == delims.end) {
            // repeated delimiter is escape sequence for the delimiter as text
            cons(
                text(str::from_char(delims.end)),
                tokenize_cs_text(delims, tailn(xs, 2u)))
          } else {
            // found an actual delimiter
            cons(
                text(str::from_chars(slice(xs, 0u, pos))),
                tokenize_cs(delims, tailn(xs, pos)))
          }
        }
      }
    }
  }

  fn tokenize_text(delims: delims, xs: [char]) -> @list<token> {
    log(debug, #fmt("tokenize_text(%s)", str::from_chars(xs)));
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
  log(debug, #fmt("parse(%s)", xs));
  let toks = token::tokenize_text(delims, str::chars(xs));
  log(info, toks);
  TODO();
}

