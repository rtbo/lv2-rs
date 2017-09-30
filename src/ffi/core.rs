
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use libc::{c_char, c_void};

pub type LV2_Handle = *mut c_void;

#[repr(C)]
pub struct LV2_Feature {
    pub URI: *const c_char,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct LV2_Descriptor {
    pub URI: *const c_char,

    pub instantiate:    Option<extern fn (descriptor: *const LV2_Descriptor,
                                          sample_rate: f64,
                                          bundle_path: *const c_char,
                                          features: *const *const LV2_Feature) -> LV2_Handle>,

    pub connect_port:   Option<extern fn (instance: LV2_Handle,
                                          port: u32,
                                          data_location: *mut c_void)>,

    pub activate:       Option<extern fn (instance: LV2_Handle)>,

    pub run:            Option<extern fn (instance: LV2_Handle, sample_count: u32)>,

    pub deactivate:     Option<extern fn (instance: LV2_Handle)>,

    pub cleanup:        Option<extern fn (instance: LV2_Handle)>,

    pub extension_data: Option<extern fn (uri: *const c_char) -> *const c_void>,
}

unsafe impl Sync for LV2_Descriptor {}
