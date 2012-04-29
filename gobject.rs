
mod raw {
  import glib::types::gpointer;
  resource object(p: gpointer) {
    nat::g_object_unref(p);
  }
  type any_object = @object;
  fn wrap_object<T>(p: gpointer, w: fn(any_object)->T) -> T {
    ret w(@object(p));
  }
  fn wrap_ref_object<T>(p: gpointer, w: fn(any_object)->T) -> T {
    nat::g_object_ref(p);
    ret wrap_object(p, w);
  }
}

#[link_name = "gobject-2.0"]
native mod nat {
  import glib::types::*;
  fn g_object_ref(p: gpointer);
  fn g_object_unref(p: gpointer);
}

