
extern crate libc;
#[macro_use]
extern crate lv2;

use std::f32;

fn db_to_coef(db: f32) -> f32 {
    if db > -90f32 {
        10f32.powf(db * 0.05f32)
    } else {
        0f32
    }
}

mod ports {
    use lv2;

    lv2_ports!(super::Amp => {
        0 => gain: lv2::meta::InputControl,
        1 => input: lv2::meta::InputAudio,
        2 => output: lv2::meta::OutputAudio
    });
}

struct Amp {}

impl<'h> lv2::Plugin<'h> for Amp {
    fn new(_sample_rate: f64, _bundle_path: &str) -> Self {
        Amp {}
    }

    fn run(&mut self, ports: &mut Self::Ports, sample_count: usize) {
        let gain = db_to_coef(*ports.gain);
        for sample in 0..sample_count {
            ports.output[sample] = ports.input[sample] * gain;
        }
    }
}

lv2_descriptor! {
    0 => DESCRIPTOR { "https://github.com/rtbo/lv2-rs/plugins/eg-amp" => Amp }
}
