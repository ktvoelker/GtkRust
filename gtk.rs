
mod priority {
  const resize: int = 110; // defined as PRIORITY_HIGH_IDLE + 10
  const redraw: int = 120; // defined as PRIORITY_HIGH_IDLE + 20
}

fn main_loop() {
  nat::gtk_main();
}

fn main_quit() {
  nat::gtk_main_quit();
}

enum widget = @rc::widget;

// TODO take a gdk::event for e
fn propagate_event(w: widget, e: glib::types::gpointer) {
  nat::gtk_propagate_event(***w, e);
}

#[link_name = "gtk-x11-2.0"]
native mod nat {
  import glib::types::*;
  fn gtk_main();
  fn gtk_main_quit();
  fn gtk_propagate_event(w: gpointer /*GtkWidget*/, e: gpointer /*GdkEvent*/);
}

mod rc {
  import glib::types::*;
  import gobject::nat::g_object_unref;
  resource widget(p: gpointer) {
    g_object_unref(p);
  }
}

