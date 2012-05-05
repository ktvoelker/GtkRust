
enum buttons {
  primary,
  middle,
  secondary
}

enum event = @{
  event_type: event_type,
  window: gpointer,
  send_event: gint8,
  time: guint32,
  state: guint,
  keyval: guint,
  length: gint,
  string: *gchar,
  hardware_keycode: guint16,
  group: guint8,
  is_modifier: guint8 // declared as a 1-bit field
};

impl of c_event for event {
  pure fn c_pointer() -> gpointer unsafe {
    ret unsafe::reinterpret_cast(*self);
  }
  pure fn c_window() -> gpointer {
    ret self.window;
  }
  pure fn c_send_event() -> gint8 {
    ret self.send_event;
  }
}

