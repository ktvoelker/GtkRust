
import io::*;

// this is a linear tree for a while
enum object = int;
enum widget = int;
enum container = int;
enum bin = int;
// now, these are all children of bin
enum window = int;
enum button = int;

iface i_container {
  fn as_container() -> container;
  fn foo();
}

impl of i_container for container {
  fn as_container() -> container {
    ret self;
  }
  fn foo() {
    println("Hello, world!");
  }
}

impl of i_container for bin {
  fn as_container() -> container {
    ret container(*self);
  }
  fn foo() {
    self.as_container().foo();
  }
}

impl of i_container for i_bin {
  fn as_container() -> container {
    ret self.as_bin().as_container();
  }
  fn foo() {
    self.as_container().foo();
  }
}

impl of i_container for window {
  fn as_container() -> container {
    ret container(*self);
  }
  fn foo() {
    self.as_container().foo();
  }
}

iface i_bin {
  fn as_bin() -> bin;
  fn bar();
}

impl of i_bin for bin {
  fn as_bin() -> bin {
    ret self;
  }
  fn bar() {
    println("Goodbye, world!");
  }
}

impl of i_bin for window {
  fn as_bin() -> bin {
    ret bin(*self);
  }
  fn bar() {
    self.as_bin().bar();
  }
}

fn main() {
  let w: window = window(5);
  w.foo();
  let c: i_container = w as i_container;
  c.foo();
  w.bar();
  let b: i_bin = w as i_bin;
  b.bar();
  let c2: i_container = b as i_container;
  c2.foo();
  // c.bar(); // should fail to compile
  // c2.bar(); // should fail to compile
}

