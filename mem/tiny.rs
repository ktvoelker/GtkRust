
import io::println;

type vp = *libc::c_void;

fn pr(n: uint) {
  println(uint::to_str(n, 10u));
}

resource foo(u: uint) {
  println("Resource freed: " + uint::to_str(u, 10u));
}

fn danger(y: vp) unsafe {
  let z: @foo = unsafe::reinterpret_cast(y);
  pr(sys::refcount(z)); // 1
}

fn main(args: [str]) unsafe {
  let x = @foo(42u);
  let x2 = x;
  pr(sys::refcount(x)); // 1
  pr(sys::refcount(x2)); // 1
  let casted = unsafe::reinterpret_cast(x);
  pr(sys::refcount(x)); // 1
  pr(sys::refcount(x2)); // 1
  unsafe::forget(x);
  pr(sys::refcount(x2)); // 1
  danger(casted);
  pr(sys::refcount(x2)); // 0
}

