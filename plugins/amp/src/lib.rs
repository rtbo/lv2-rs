
extern crate libc;
#[macro_use]
extern crate lv2;

use lv2::ffi::LV2_Descriptor;

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

    fn run(&mut self, ports: &mut AmpPorts, sample_count: usize) {
        let gain = db_to_coef(ports.gain);
        for sample in 0 .. sample_count {
            ports.output[sample] = ports.input[sample] * gain;
        }
    }
}


lv2_descriptor! (
    DESCRIPTOR1 { "https://github.com/rtbo/lv2-rs/plugins/eg-amp" => Amp }
);


// static DESCRIPTOR: LV2_Descriptor = LV2_Descriptor {
//     URI: b"https://github.com/rtbo/lv2-rs/plugins/eg-amp\0" as *const u8 as _,
//     instantiate: Some(lv2::instantiate::<Amp>),
//     connect_port: Some(lv2::connect_port::<Amp>),
//     activate: Some(lv2::activate::<Amp>),
//     run: Some(lv2::run::<Amp>),
//     deactivate: Some(lv2::deactivate::<Amp>),
//     cleanup: Some(lv2::cleanup::<Amp>),
//     extension_data: None,
// };

// #[no_mangle]
// pub extern "C" fn lv2_descriptor(index: u32) -> *const LV2_Descriptor
// {
//     match index {
//         0 => { &DESCRIPTOR },
//         _ => { ptr::null() }
//     }
// }