
use template;

fn main() {
  let t = template::parse(
      template::default_delims(),
      "foo $bar$ $if exists quux$ baz $end # the end$");
}

