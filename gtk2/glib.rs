
import libc::*;

mod priority {
  const high: int = -100;
  const default: int = 0;
  const high_idle: int = 100;
  const default_idle: int = 200;
  const low: int = 300;
}

enum event_source = u32;

mod cb {
  import glib::types::*;

  fn pack_closure<T, U>(func: fn@(@mut T)->U, data: @mut T) -> gpointer unsafe {
    let c = fn@() -> U {
      ret func(data);
    };
    let p = @c;
    let d: gpointer = unsafe::reinterpret_cast(p);
    unsafe::forget(p);
    ret d;
  }

  type source_func<T> = fn@(@mut T)->bool;

  fn source_func() -> gpointer unsafe {
    ret unsafe::reinterpret_cast(raw::source_func);
  }

  mod raw {

    crust fn source_func(data: gpointer) -> gboolean unsafe {
      let p: @fn@()->bool = unsafe::reinterpret_cast(data);
      let r = (*p)();
      if (r) {
        // The callback is going to be executed again, so save it.
        unsafe::forget(p);
      }
      ret gboolean::from_bool(r);
    }

  }
}

mod idle {

  fn add<T>(func: cb::source_func<T>, data: @mut T) -> event_source {
    ret event_source(nat::g_idle_add(cb::source_func(), cb::pack_closure(func, data)));
  }

  fn add_full<T>(priority: i32, func: cb::source_func<T>, data: @mut T)
    -> event_source {
      ret event_source(
          nat::g_idle_add_full(
            priority,
            cb::source_func(),
            cb::pack_closure(func, data),
            ptr::null()));
    }

}

mod timeout {

  fn add<T>(interval: u32, func: cb::source_func<T>, data: @mut T)
    -> event_source {
    ret event_source(
        nat::g_timeout_add(
          interval,
          cb::source_func(),
          cb::pack_closure(func, data)));
  }

  fn add_full<T>(
      priority: i32,
      interval: u32,
      func: cb::source_func<T>,
      data: @mut T)
    -> event_source {
      ret event_source(
          nat::g_timeout_add_full(
            priority,
            interval,
            cb::source_func(),
            cb::pack_closure(func, data),
            ptr::null()));
    }

  fn add_seconds<T>(interval: u32, func: cb::source_func<T>, data: @mut T)
    -> event_source {
    ret event_source(
        nat::g_timeout_add_seconds(
          interval,
          cb::source_func(),
          cb::pack_closure(func, data)));
  }

  fn add_seconds_full<T>(
      priority: i32,
      interval: u32,
      func: cb::source_func<T>,
      data: @mut T)
    -> event_source {
      ret event_source(
          nat::g_timeout_add_seconds_full(
            priority,
            interval,
            cb::source_func(),
            cb::pack_closure(func, data),
            ptr::null()));
    }
}

fn check_version(major: u32, minor: u32, micro: u32) {
  let err_str = nat::glib_check_version(major, minor, micro);
  if err_str == ptr::null() {
    ret;
  } else unsafe {
    log(error, str::unsafe::from_c_str(err_str));
    fail;
  }
}

mod signal {
  import types::*;

  enum timing {
    before,
    after
  }

  enum handler_id = gulong;

  fn connect<T: gobject::i_object>(
      obj: T,
      name: str,
      callback: gpointer,
      data: gpointer,
      when: timing) -> handler_id {
    ret handler_id(
          str::as_c_str(name, {|c_name|
            nat::g_signal_connect_data(
              obj.c_object(),
              c_name,
              callback,
              data,
              ptr::null(),
              if (when == before) { 0i32 } else { 1i32 } /* typed as GConnectFlags */)
          }));
  }

}

#[link_name = "glib-2.0"]
native mod nat {
  import types::*;
  fn glib_check_version(major: guint, minor: guint, micro: guint) -> *gchar;
  fn g_timeout_add(interval: guint, func: gpointer, data: gpointer) -> guint;
  fn g_timeout_add_full(
      priority: gint,
      interval: guint,
      func: gpointer,
      data: gpointer,
      on_destroy: gpointer)
    -> guint;
  fn g_timeout_add_seconds(interval: guint, func: gpointer, data: gpointer) -> guint;
  fn g_timeout_add_seconds_full(
      priority: gint,
      interval: guint,
      func: gpointer,
      data: gpointer,
      on_destroy: gpointer)
    -> guint;
  fn g_idle_add(func: gpointer, data: gpointer) -> guint;
  fn g_idle_add_full(
      priority: gint, func: gpointer, data: gpointer, on_destroy: gpointer) -> guint;
  fn g_signal_connect_data(
      obj: gpointer,
      name: *gchar,
      callback: gpointer,
      data: gpointer,
      on_destroy: gpointer,
      flags: gint) -> gulong;
}

mod types {

  mod gboolean {
    fn from_bool(b: bool) -> gboolean {
      ret if b { 1i32 } else { 0i32 };
    }
  }

  type gboolean = gint;
  type gpointer = *c_void;
  type gchar = c_char;
  type guchar = c_uchar;
  type gint = c_int;
  type guint = c_uint;
  type gshort = c_short;
  type gushort = c_ushort;
  type glong = c_long;
  type gulong = c_ulong;
  type gint8 = i8;
  type guint8 = u8;
  type gint16 = i16;
  type guint16 = u16;
  type gint32 = i32;
  type guint32 = u32;
  type gint64 = i64;
  type guint64 = u64;
  type gfloat = c_float;
  type gdouble = c_double;
  type gsize = c_ulong;
  type gssize = c_long;
  type goffset = gint64;
  type gintptr = c_long;
  type guintptr = c_ulong;

}

