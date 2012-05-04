
import io::*;

iface foo {
  fn foo() -> int;
}

iface bar {
  fn bar() -> int;
}

impl of foo for int {
  fn foo() -> int {
    ret self * 2;
  }
}

impl of bar for int {
  fn bar() -> int {
    ret self * 3;
  }
}

impl of bar for foo {
  fn bar() -> int {
    ret self.foo() * 5;
  }
}

fn baz<T: bar>(n: T) -> int {
  ret n.bar();
}

fn main() {
  // prints 2
  println(int::to_str(1.foo(), 10u));
  // prints 3 if it calls the impl for int
  // prints 10 if it calls the impl for foo
  println(int::to_str(1.bar(), 10u));
  println(int::to_str(baz(1), 10u));
}

