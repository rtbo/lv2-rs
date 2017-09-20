
extern crate libc;
extern crate lv2;

use libc::{c_char, c_void};
use lv2::ffi::core::*;

use std::boxed::Box;
use std::mem::transmute;
use std::ptr;
use std::slice;
use std::f32;

const AMP_GAIN: u32 = 0;
const AMP_INPUT: u32 = 1;
const AMP_OUTPUT: u32 = 2;

struct Amp {
    gain: *const f32,
    input: *const f32,
    output: *mut f32,
}

fn db_to_coef(db: f32) -> f32
{
    if db > -90f32 {
        10f32.powf(db * 0.05f32)
    }
    else {
        0f32
    }
}

extern fn instantiate (_descriptor: *const LV2_Descriptor,
                       _sample_rate: f64,
                       _bundle_path: *const c_char,
                       _features: *const LV2_Feature) -> LV2_Handle
{
    let amp = Box::new(Amp {
        gain: ptr::null(),
        input: ptr::null(),
        output: ptr::null_mut(),
    });
    Box::into_raw(amp) as LV2_Handle
}

extern fn connect_port (instance: LV2_Handle,
                        port: u32,
                        data_location: *mut c_void)
{
    let mut amp: &mut Amp = unsafe { transmute(instance) };
    match port {
        AMP_GAIN => { amp.gain = data_location as *const f32; },
        AMP_INPUT => { amp.input = data_location as *const f32; },
        AMP_OUTPUT => { amp.output = data_location as *mut f32; },
        _ => {},
    }
}

extern fn run (instance: LV2_Handle, sample_count: u32)
{
    let amp: &Amp = unsafe { transmute(instance) };
    let gain = db_to_coef(unsafe { *amp.gain });
    let input = unsafe { slice::from_raw_parts(amp.input, sample_count as usize) };
    let output = unsafe { slice::from_raw_parts_mut(amp.output, sample_count as usize) };
    for sample in 0 .. sample_count as usize {
        output[sample] = input[sample] * gain;
    }
}

// pub deactivate:     extern fn (instance: LV2_Handle),


extern fn cleanup(instance: LV2_Handle)
{
    unsafe { let _ = Box::from_raw(instance as *mut Amp); }
}

static DESCRIPTOR: LV2_Descriptor = LV2_Descriptor {
    URI: b"https://github.com/rtbo/lv2-rs/plugins/eg-amp-ffi\0" as *const u8 as _,
    instantiate: Some(instantiate),
    connect_port: Some(connect_port),
    activate: None,
    run: Some(run),
    deactivate: None,
    cleanup: Some(cleanup),
    extension_data: None,
};


#[no_mangle]
pub extern "C" fn lv2_descriptor(index: u32) -> *const LV2_Descriptor
{
    match index {
        0 => { &DESCRIPTOR },
        _ => { ptr::null() }
    }
}
