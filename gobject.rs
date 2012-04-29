
#[link_name = "gobject-2.0"]
native mod nat {
  import glib::types::*;
  fn g_object_unref(o: gpointer);
}

