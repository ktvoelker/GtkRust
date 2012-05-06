
use templates;
import templates::*;

fn main() {
  let t = template::parse(
      "foo $bar$ $if exists quux$ baz $end # the end$");
}

