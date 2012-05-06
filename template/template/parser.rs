
fn parse(delims: lexer::delims, xs: str) -> template {
  log(debug, #fmt("parse(%s)", xs));
  let toks = lexer::tokenize_text(delims, str::chars(xs));
  log(info, toks);
  TODO();
}

