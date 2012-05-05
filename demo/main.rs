
use gtk2;
import gtk2::*;
import gtk::window::window;
import gtk::window::i_widget;
import gtk::window::i_object;
import gtk::widget::widget;

import io::*;

crust fn my_callback(
    _w: glib::types::gpointer,
    _e: glib::types::gpointer,
    _p: glib::types::gpointer) -> glib::types::gboolean {
  gtk::main_quit();
  ret glib::types::gboolean::from_bool(false);
}

fn main() unsafe {
  gtk::init();
  let win = gtk::window::create(gtk::window::toplevel);
  win.set_title("Hello, world!");
  glib::signal::connect(
      win,
      "delete-event",
      unsafe::reinterpret_cast(my_callback),
      ptr::null(),
      glib::signal::before);
  win.as_widget().show();
  gtk::main_loop();
}

