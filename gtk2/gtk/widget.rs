
import glib::types::*;
import gobject::object;

enum widget = gobject::raw::any_object;

iface i_widget {
  fn as_widget() -> widget;
}

impl of i_widget for widget {
  fn as_widget() -> widget {
    ret self;
  }
}

impl of gobject::object for widget {
  pure fn c_object() -> gpointer {
    ret (*self).c_object();
  }
}

impl widget for widget {
  fn propagate_event<T: gdk::event::c_event>(e: T) {
    nat::gtk_propagate_event(self.c_object(), e.c_pointer());
  }

  fn show() {
    nat::gtk_widget_show(self.c_object());
  }

  fn hide() {
    nat::gtk_widget_hide(self.c_object());
  }
}

#[link_name = "gtk-x11-2.0"]
native mod nat {
  fn gtk_propagate_event(w: gpointer /*GtkWidget*/, e: gpointer /*GdkEvent*/);
  fn gtk_widget_show(w: gpointer /*GtkWidget*/);
  fn gtk_widget_hide(w: gpointer /*GtkWidget*/);
}

