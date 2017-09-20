
extern crate libc;
extern crate lv2;

use libc::{c_char, c_void};

use lv2::ffi::*;
use lv2::Plugin;

use std::boxed::Box;
use std::ffi::{CStr};
use std::marker;
use std::mem::transmute;
use std::ptr;
use std::slice;
use std::f32;

const AMP_GAIN: u32 = 0;
const AMP_INPUT: u32 = 1;
const AMP_OUTPUT: u32 = 2;

struct AmpPorts<'a> {
    gain: f32,
    input: &'a [f32],
    output: &'a mut [f32],
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

struct Amp<'a> {
    _marker: marker::PhantomData<&'a ()>,
}

unsafe impl<'a> lv2::Ported for Amp<'a> {
    type Ports = AmpPorts<'a>;
}

impl<'a> lv2::Plugin for Amp<'a>
{
    fn new (_sample_rate: f64, _bundle_path: &str) -> Self {
        Amp {
            _marker: marker::PhantomData
        }
    }

    fn run(&mut self, ports: &mut AmpPorts<'a>, sample_count: usize) {
        let gain = db_to_coef(ports.gain);
        for sample in 0 .. sample_count {
            ports.output[sample] = ports.input[sample] * gain;
        }
    }
}

struct AmpPortsRaw {
    gain: *const f32,
    input: *const f32,
    output: *mut f32,
}

struct AmpInstance<'a> {
    ports: AmpPortsRaw,
    state: Amp<'a>,
}

extern fn instantiate (_descriptor: *const LV2_Descriptor,
                       sample_rate: f64,
                       bundle_path: *const c_char,
                       _features: *const LV2_Feature) -> LV2_Handle
{
    let amp_ports = AmpPortsRaw {
        gain: ptr::null(),
        input: ptr::null(),
        output: ptr::null_mut(),
    };

    let bundle_path = unsafe { CStr::from_ptr(bundle_path).to_str().unwrap() };

    let instance = Box::new(
        AmpInstance {
            ports: amp_ports,
            state: Amp::new(sample_rate, bundle_path),
        }
    );
    Box::into_raw(instance) as LV2_Handle
}

extern fn connect_port (instance: LV2_Handle,
                        port: u32,
                        data_location: *mut c_void)
{
    let mut instance: &mut AmpInstance = unsafe { transmute(instance) };

    match port {
        AMP_GAIN => { instance.ports.gain = data_location as *const f32; },
        AMP_INPUT => { instance.ports.input = data_location as *const f32; },
        AMP_OUTPUT => { instance.ports.output = data_location as *mut f32; },
        _ => {},
    }
}

extern fn run (instance: LV2_Handle, sample_count: u32)
{
    let amp: &mut AmpInstance = unsafe { transmute(instance) };

    let mut amp_ports = AmpPorts {
        gain: db_to_coef(unsafe { *amp.ports.gain }),
        input: unsafe { slice::from_raw_parts(amp.ports.input, sample_count as usize) },
        output: unsafe { slice::from_raw_parts_mut(amp.ports.output, sample_count as usize) },
    };

    amp.state.run(&mut amp_ports, sample_count as usize);
}

extern fn cleanup(instance: LV2_Handle)
{
    unsafe { let _ = Box::from_raw(instance as *mut AmpInstance); }
}

static DESCRIPTOR: LV2_Descriptor = LV2_Descriptor {
    URI: b"https://github.com/rtbo/lv2-rs/plugins/eg-amp\0" as *const u8 as _,
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