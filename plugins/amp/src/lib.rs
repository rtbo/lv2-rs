
extern crate libc;
extern crate lv2;

use libc::{c_char, c_void};

use lv2::ffi::*;
use lv2::Plugin;

use std::boxed::Box;
use std::ffi::{CStr};
use std::mem::transmute;
use std::ptr;
use std::slice;
use std::f32;

const AMP_GAIN: usize = 0;
const AMP_INPUT: usize = 1;
const AMP_OUTPUT: usize = 2;

struct AmpPorts<'h> {
    gain: f32,
    input: &'h [f32],
    output: &'h mut [f32],
}

#[derive(Debug, Copy, Clone)]
struct AmpPortsRaw {
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

struct Amp { }

unsafe impl<'h> lv2::Ported<'h> for Amp
{
    type Ports = AmpPorts<'h>;
    type PortsRaw = AmpPortsRaw;

    fn new_ports_raw() -> Self::PortsRaw {
        AmpPortsRaw {
            gain: ptr::null(),
            input: ptr::null(),
            output: ptr::null_mut(),
        }
    }

    fn connect_port(port: usize, data: *mut (), ports_raw: &mut Self::PortsRaw) {
        match port {
            AMP_GAIN => { ports_raw.gain = data as *const f32; },
            AMP_INPUT => { ports_raw.input = data as *const f32; },
            AMP_OUTPUT => { ports_raw.output = data as *mut f32; },
            _ => {},
        }

    }

    fn convert_ports(ports_raw: Self::PortsRaw, sample_count: usize) -> Self::Ports {
        AmpPorts {
            gain: unsafe { *ports_raw.gain },
            input: unsafe { slice::from_raw_parts(ports_raw.input, sample_count as usize) },
            output: unsafe { slice::from_raw_parts_mut(ports_raw.output, sample_count as usize) },
        }
    }
}

impl<'h> lv2::Plugin<'h> for Amp
{
    fn new (_sample_rate: f64, _bundle_path: &str) -> Self {
        Amp { }
    }

    fn run(&mut self, ports: &'h mut AmpPorts, sample_count: usize) {
        let gain = db_to_coef(ports.gain);
        for sample in 0 .. sample_count {
            ports.output[sample] = ports.input[sample] * gain;
        }
    }
}


struct AmpInstance {
    ports_raw: AmpPortsRaw,
    state: Amp,
}

extern fn instantiate (_descriptor: *const LV2_Descriptor,
                       sample_rate: f64,
                       bundle_path: *const c_char,
                       _features: *const LV2_Feature) -> LV2_Handle
{
    let amp_ports = <Amp as lv2::Ported>::new_ports_raw();

    let bundle_path = unsafe { CStr::from_ptr(bundle_path).to_str().unwrap() };

    let instance = Box::new(
        AmpInstance {
            ports_raw: amp_ports,
            state: Amp::new(sample_rate, bundle_path),
        }
    );
    Box::into_raw(instance) as LV2_Handle
}

extern fn connect_port (instance: LV2_Handle,
                        port: u32,
                        data_location: *mut c_void)
{
    let instance: &mut AmpInstance = unsafe { transmute(instance) };
    <Amp as lv2::Ported>::connect_port(port as usize,
                                       data_location as *mut _,
                                       &mut instance.ports_raw);
}

extern fn run (instance: LV2_Handle, sample_count: u32)
{
    let amp: &mut AmpInstance = unsafe { transmute(instance) };

    let sample_count = sample_count as usize;

    let mut amp_ports = <Amp as lv2::Ported>::convert_ports(amp.ports_raw, sample_count);
    amp.state.run(&mut amp_ports, sample_count);
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