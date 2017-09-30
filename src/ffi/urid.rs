#![allow(non_camel_case_types)]

use libc::{c_void, c_char};

pub type LV2_URID_Map_Handle = *mut c_void;

pub type LV2_URID_Unmap_Handle = *mut c_void;

pub type LV2_URID = u32;

#[repr(C)]
pub struct LV2_URID_Map {
    pub handle: LV2_URID_Map_Handle,
    pub map:    Option<extern fn (handle: LV2_URID_Map_Handle, uri: *const c_char) -> LV2_URID>,
}

#[repr(C)]
pub struct LV2_URID_Unmap {
	pub handle: LV2_URID_Unmap_Handle,
    pub unmap:  Option<extern fn (handle: LV2_URID_Unmap_Handle, urid: LV2_URID) -> *const c_char>,
}
