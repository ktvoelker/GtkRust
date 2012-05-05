
import glib::types::*;

iface object {
  pure fn c_object() -> gpointer;
}

impl of object for raw::any_object {
  pure fn c_object() -> gpointer {
    ret **self;
  }
}

mod raw {
  resource object(p: gpointer) {
    nat::g_object_unref(p);
  }
  type any_object = @object;
  fn wrap_object(p: gpointer) -> any_object {
    ret @object(p);
  }
  fn wrap_and_sink_object(p: gpointer) -> any_object {
    nat::g_object_ref_sink(p);
    ret wrap_object(p);
  }
}

#[link_name = "gobject-2.0"]
native mod nat {
  fn g_object_ref(p: gpointer);
  fn g_object_unref(p: gpointer);
  fn g_object_ref_sink(p: gpointer) -> gpointer;
}

