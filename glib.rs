
import libc::*;
import io::writer_util;

mod priority {
  const high: int = -100;
  const default: int = 0;
  const high_idle: int = 100;
  const default_idle: int = 200;
  const low: int = 300;
}

enum event_source = u32;

mod timeout {
  mod raw {
    import glib::types::*;
    fn pack_closure<T>(func: fn@(@T)->bool, data: @T) -> gpointer unsafe {
      let c = fn@() -> bool {
        ret func(data);
      };
      let p = @c;
      let d: gpointer = unsafe::reinterpret_cast(p);
      unsafe::forget(p);
      ret d;
    }
    crust fn c_callback(data: gpointer) -> gboolean unsafe {
      let p: @fn@()->bool = unsafe::reinterpret_cast(data);
      ret gboolean::from_bool((*p)());
    }
  }
  fn add<T>(interval: u32, func: fn@(@T)->bool, data: @T) -> event_source unsafe {
    ret event_source(
        nat::g_timeout_add(
          interval,
          unsafe::reinterpret_cast(raw::c_callback),
          raw::pack_closure(func, data)));
  }
}

fn check_version(major: u32, minor: u32, micro: u32) {
  let err_str = nat::glib_check_version(major, minor, micro);
  if err_str == ptr::null() {
    ret;
  } else unsafe {
    io::stderr().write_line(str::unsafe::from_c_str(err_str));
    fail;
  }
}

#[link_name = "glib-2.0"]
native mod nat {
  import types::*;
  fn glib_check_version(major: guint, minor: guint, micro: guint) -> *gchar;
  fn g_timeout_add(interval: guint, func: gpointer, data: gpointer) -> guint;
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

