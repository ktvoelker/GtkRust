
mod priority {
  const resize: int = 110; // defined as PRIORITY_HIGH_IDLE + 10
  const redraw: int = 120; // defined as PRIORITY_HIGH_IDLE + 20
}

// TODO
// Use typestate system to enforce proper GTK initialization.

// TODO
// Support passing in args and getting back modified args.
fn init() {
  nat::gtk_init(ptr::null(), ptr::null());
}

fn main_loop() {
  nat::gtk_main();
}

fn main_quit() {
  nat::gtk_main_quit();
}

enum widget = gobject::raw::any_object;

// TODO take a gdk::event for e
fn propagate_event(w: widget, e: glib::types::gpointer) {
  nat::gtk_propagate_event(***w, e);
}

#[link_name = "gtk-x11-2.0"]
native mod nat {
  import glib::types::*;
  fn gtk_init(argc: *libc::c_int, argv: ***libc::c_char);
  fn gtk_main();
  fn gtk_main_quit();
  fn gtk_propagate_event(w: gpointer /*GtkWidget*/, e: gpointer /*GdkEvent*/);
}

