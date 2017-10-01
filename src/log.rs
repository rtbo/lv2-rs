
use ::core::{Feature, RawFeature};
use ::urid;
use ::ffi::log::*;

use libc::{c_char};
use std::ffi::CString;
use std::marker;
use std::mem;

pub const URI: &'static str = "http://lv2plug.in/ns/ext/log";

pub mod class {
    pub const ENTRY   : &'static str = "http://lv2plug.in/ns/ext/log#Entry";
    pub const ERROR   : &'static str = "http://lv2plug.in/ns/ext/log#Error";
    pub const NOTE    : &'static str = "http://lv2plug.in/ns/ext/log#Note";
    pub const TRACE   : &'static str = "http://lv2plug.in/ns/ext/log#Trace";
    pub const WARNING : &'static str = "http://lv2plug.in/ns/ext/log#Warning";
}
pub mod inst {
    pub const LOG     : &'static str = "http://lv2plug.in/ns/ext/log#log";
}


pub struct Log<'h> {
    raw: *const LV2_Log,
    marker: marker::PhantomData<&'h ()>,
}

impl<'h> Feature<'h> for Log<'h> {
    fn uri() -> &'static str {
        inst::LOG
    }
    unsafe fn from_raw(raw: &RawFeature<'h>) -> Self {
        debug_assert!(raw.uri() == Self::uri());
        Log {
            raw: mem::transmute((*raw.raw).data),
            marker: marker::PhantomData,
        }
    }
}

impl<'h> Log<'h> {
    pub fn print(&self, urid: urid::URID, msg: String) {
        let cstr = CString::new(msg).unwrap();
        unsafe {
            if let Some(func) = (*self.raw).printf {
                let _ = func((*self.raw).handle,
                             urid,
                             "%s\0".as_ptr() as *const c_char,
                             cstr.as_ptr());
            }
        }
    }
    pub fn println(&self, urid: urid::URID, msg: String) {
        let cstr = CString::new(msg).unwrap();
        unsafe {
            if let Some(func) = (*self.raw).printf {
                let _ = func((*self.raw).handle,
                             urid,
                             "%s\n\0".as_ptr() as *const c_char,
                             cstr.as_ptr());
            }
        }
    }
}

#[macro_export]
macro_rules! lv2_log {
    ($log:expr, $urid:expr, $fmt:expr $(, $arg:tt)*) => {
        $log.println(urid, format!( $fmt, $($arg),* ));
    };
    (@error, $log:expr, $fmt:expr $(, $arg:tt)*) => {
        $log.println_error(format!( $fmt, $($arg),* ));
    };
    (@note, $log:expr, $fmt:expr $(, $arg:tt)*) => {
        $log.println_note(format!( $fmt, $($arg),* ));
    };
    (@trace, $log:expr, $fmt:expr $(, $arg:tt)*) => {
        $log.println_trace(format!( $fmt, $($arg),* ));
    };
    (@warning, $log:expr, $fmt:expr $(, $arg:tt)*) => {
        $log.println_warning(format!( $fmt, $($arg),* ));
    };
}

pub struct Logger<'h> {
    log: &'h Log<'h>,

    error: urid::URID,
    note: urid::URID,
    trace: urid::URID,
    warning: urid::URID,
}

impl<'h> Logger<'h> {
    pub fn new_with_map(log: &'h Log, map: &'h urid::Map) -> Logger<'h> {
        Logger {
            log: log,
            error: map.map(class::ERROR),
            note: map.map(class::NOTE),
            trace: map.map(class::TRACE),
            warning: map.map(class::WARNING),
        }
    }
    pub fn new(log: &'h Log) -> Logger<'h> {
        Logger {
            log: log,
            error: 0, note: 0, trace: 0, warning: 0
        }
    }

    pub fn println(&self, urid: urid::URID, msg: String) {
        self.log.println(urid, msg);
    }

    pub fn println_error(&self, msg: String) {
        self.log.println(self.error, msg);
    }

    pub fn println_note(&self, msg: String) {
        self.log.println(self.note, msg);
    }

    pub fn println_trace(&self, msg: String) {
        self.log.println(self.trace, msg);
    }

    pub fn println_warning(&self, msg: String) {
        self.log.println(self.warning, msg);
    }
}
