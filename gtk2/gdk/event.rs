
enum propagation {
  propagate,
  stop
}

enum window = gpointer;

iface event {
  pure fn window() -> window;
  pure fn send_event() -> bool;
}

iface c_event {
  pure fn c_pointer() -> gpointer;
  pure fn c_window() -> gpointer;
  pure fn c_send_event() -> gint8;
}

impl of event for c_event {
  pure fn window() -> window {
    ret window(self.c_window());
  }
  pure fn send_event() -> bool {
    ret if self.c_send_event() == 0i8 { false } else { true };
  }
}

enum event_type {
  nothing		         = -1,
  delete		         =  0,
  destroy		         =  1,
  expose		         =  2,
  motion_notify      =  3,
  button_press       =  4,
  two_button_press   =  5,
  three_button_press =  6,
  button_release     =  7,
  key_press		       =  8,
  key_release        =  9,
  enter_notify       = 10,
  leave_notify       = 11,
  focus_change       = 12,
  configure		       = 13,
  map		             = 14,
  unmap		           = 15,
  property_notify    = 16,
  selection_clear    = 17,
  selection_request  = 18,
  selection_notify   = 19,
  proximity_in       = 20,
  proximity_out      = 21,
  drag_enter         = 22,
  drag_leave         = 23,
  drag_motion        = 24,
  drag_status        = 25,
  drop_start         = 26,
  drop_finished      = 27,
  client_event       = 28,
  visibility_notify  = 29,
  scroll             = 31,
  window_state       = 32,
  setting            = 33,
  owner_change       = 34,
  grab_broken        = 35,
  damage             = 36,
  touch_begin        = 37,
  touch_update       = 38,
  touch_end          = 39,
  touch_cancel       = 40
}

