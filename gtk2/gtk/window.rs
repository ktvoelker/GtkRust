
import gobject::object;
import gobject::i_object;

enum window = gobject::raw::any_object;

impl of gobject::i_object for window {
  pure fn c_object() -> gpointer {
    ret (*self).c_object();
  }
  fn as_object() -> object {
    ret gobject::object(*self);
  }
}

impl of gtk::widget::i_widget for window {
  fn as_widget() -> gtk::widget::widget {
    ret gtk::widget::widget(*self);
  }
}

enum window_type {
  toplevel = 0,
  popup    = 1
}

fn create(t: window_type) -> window {
  ret window(gobject::raw::wrap_and_sink_object(nat::gtk_window_new(t as libc::c_int)));
}

impl window for window {

  fn set_title(t: str) {
    str::as_c_str(t, bind nat::gtk_window_set_title(self.c_object(), _));
  }

}

#[link_name = "gtk-x11-2.0"]
native mod nat {
  fn gtk_window_new(t: libc::c_int /*GtkWindowType*/) -> gpointer;
  fn gtk_window_set_title(w: gpointer /*GtkWindow*/, t: *libc::c_char);
}

