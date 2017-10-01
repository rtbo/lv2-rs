
#![allow(non_camel_case_types)]

use ::urid;

use libc::{c_char, c_int, c_void};

pub type LV2_Log_Handle = *mut c_void;

#[repr(C)]
pub struct LV2_Log {
    pub handle: LV2_Log_Handle,
    pub printf: Option<extern "C" fn (handle: LV2_Log_Handle,
                                  urid: urid::URID,
                                  fmt: *const c_char, ...) -> c_int>,
    // unused fn pointer
    _pad: usize,
}
