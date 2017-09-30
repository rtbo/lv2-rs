
use ::ffi::urid::*;
use ::{Feature, RawFeature};

use std::ffi::{CStr, CString};
use std::marker;
use std::mem;

pub const URI: &'static str = "http://lv2plug.in/ns/ext/urid";

pub mod inst {
    pub const MAP:   &'static str = "http://lv2plug.in/ns/ext/urid#map";
    pub const UNMAP: &'static str = "http://lv2plug.in/ns/ext/urid#unmap";
}

pub type URID = u32;

pub struct Map<'h> {
    raw: *const LV2_URID_Map,
    marker: marker::PhantomData<&'h ()>,
}

impl<'h> Feature<'h> for Map<'h>
{
    fn uri() -> &'static str {
        inst::MAP
    }
    unsafe fn from_raw(raw: RawFeature<'h>) -> Self {
        debug_assert!(raw.uri() == Self::uri());
        Map {
            raw: mem::transmute((*raw.raw).data),
            marker: marker::PhantomData,
        }
    }
}

impl<'h> Map<'h> {
    pub fn map (&self, uri: &str) -> URID {
        let uri = CString::new(uri).unwrap();
        if let Some(func) = unsafe { (*self.raw).map } {
            unsafe { func((*self.raw).handle, uri.as_ptr()) }
        }
        else {
            0
        }
    }
}


pub struct Unmap<'h> {
    raw: *const LV2_URID_Unmap,
    marker: marker::PhantomData<&'h ()>,
}

impl<'h> Feature<'h> for Unmap<'h>
{
    fn uri() -> &'static str {
        inst::UNMAP
    }
    unsafe fn from_raw(raw: RawFeature<'h>) -> Self {
        debug_assert!(raw.uri() == Self::uri());
        Unmap {
            raw: mem::transmute(raw.raw),
            marker: marker::PhantomData,
        }
    }
}

impl<'h> Unmap<'h> {
    pub fn unmap (&self, id: URID) -> Option<&str> {
        if let Some(func) = unsafe { (*self.raw).unmap } {
            let cstr = unsafe {
                CStr::from_ptr( func((*self.raw).handle, id) )
            };
            cstr.to_str().ok()
        }
        else {
            None
        }
    }
}
