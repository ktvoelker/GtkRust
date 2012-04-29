
import io::*;

fn main(args : [str]) {
  io::println("Hello, world!");
  gtk::init();
  glib::check_version(2u32, 30u32, 2u32);
  let x: @mut int = @mut 0;
  glib::timeout::add(10u32, fn@(y: @mut int) -> bool {
    println(int::to_str(*y, 10u));
    *y += 1;
    if (*y == 10) {
      glib::timeout::add(10u32, fn@(y: @mut int) -> bool {
        println(int::to_str(*y, 10u));
        gtk::main_quit();
        ret false;
      }, y);
    }
    ret *y < 10;
  }, x);
  gtk::main_loop();
}

