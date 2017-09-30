
#[macro_use]
extern crate lv2;

use lv2::midi;
use lv2::urid::{self, URID};

use std::f64;


const ATTACK_LEN: f32 = 0.1;
const DECAY_LEN: f32 = 0.1;
const SUSTAIN_GAIN: f32 = 0.85;
const RELEASE_LEN: f32 = 0.1;

#[derive(Copy, Clone)]
struct Enveloppe {
    attack_slp: f32,
    attack_gain: f32,
    decay_slp: f32,
    sustain_gain: f32,
    release_slp: f32,
}

impl Enveloppe {
    fn new(vel: u8, sample_rate: f64) -> Enveloppe {
        let gain = vel as f32 / 127_f32;
        Enveloppe {
            attack_slp: gain / (ATTACK_LEN * sample_rate as f32),
            attack_gain: gain,
            decay_slp: (gain * (SUSTAIN_GAIN - 1_f32)) / (DECAY_LEN * sample_rate as f32),
            sustain_gain: gain * SUSTAIN_GAIN,
            release_slp: - gain * SUSTAIN_GAIN / (RELEASE_LEN * sample_rate as f32),
        }
    }
}

#[derive(Copy, Clone)]
enum State {
    Attack, Decay, Sustain, Release,
}


fn key_freq(key: u8) -> f64 {
    2_f64.powf((key as f64 - 69_f64)/12_f64) * 440_f64
}

fn key_pulse(key: u8, sample_rate: f64) -> f64 {
    2_f64 * f64::consts::PI * key_freq(key) / sample_rate
}

#[derive(Copy, Clone)]
struct Voice {
    key: u8,
    phase: f64, // rad
    pulse: f64, // rad / sample
    gain: f32,
    enveloppe: Enveloppe,
    state: State,
}

impl Voice {
    fn new(synth: &Synth, key: u8, vel: u8) -> Voice {
        Voice {
            key: key,
            phase: 0_f64,
            pulse: synth.pulses[key as usize],
            gain: 0_f32,
            enveloppe: Enveloppe::new(vel, synth.sample_rate),
            state: State::Attack,
        }
    }
    fn run(&mut self, output: &mut [f32], start: usize, end: usize) -> bool {
        let mut released = false;
        for s in start .. end {
            output[s] += self.phase.sin() as f32 * self.gain;
            self.phase += self.pulse;

            match self.state {
                State::Attack => {
                    self.gain += self.enveloppe.attack_slp;
                    if self.gain >= self.enveloppe.attack_gain {
                        self.gain = self.enveloppe.attack_gain;
                        self.state = State::Decay;
                    }
                },
                State::Decay => {
                    self.gain += self.enveloppe.decay_slp;
                    if self.gain <= self.enveloppe.sustain_gain {
                        self.gain = self.enveloppe.sustain_gain;
                        self.state = State::Sustain;
                    }
                },
                State::Sustain => {},
                State::Release => {
                    self.gain += self.enveloppe.release_slp;
                    if self.gain <= 0_f32 {
                        self.gain = 0_f32;
                        released = true;
                        break;
                    }
                }

            }
        }
        released
    }
}

struct Synth {
    sample_rate: f64,
    midi_event: URID,
    voices: [Option<Voice>; 32],
    pulses: [f64; 128],
}

mod ports {
    use lv2;
    use lv2::atom;

    lv2_ports!(super::Synth => {
        0 => input: atom::meta::InputSequence,
        1 => output: lv2::meta::OutputAudio
    });
}

impl<'h> lv2::Plugin<'h> for Synth {

    fn new(sample_rate: f64, _bundle_path: &str, features: lv2::FeatureList<'h>) -> Option<Self> {
        let mut me = 0;
        for f in features {
            if f.uri() == <urid::Map as lv2::Feature<'h>>::uri() {
                let map = unsafe {
                    <urid::Map as lv2::Feature<'h>>::from_raw(f)
                };
                me = map.map(midi::class::MIDIEVENT);
            }
        }
        if me > 0 {
            let mut synth = Synth {
                sample_rate: sample_rate,
                midi_event: me,
                voices: [None; 32],
                pulses: [0_f64; 128],
            };
            for key in 0..128 {
                synth.pulses[key] = key_pulse(key as u8, sample_rate);
            }
            Some(synth)
        }
        else {
            None
        }
    }

    fn run(&mut self, ports: &mut Self::Ports, sample_count: usize) {
        let mut offset = 0;
        for ev in ports.input.iter() { // ev has type atom::Event
            if ev.type_urid() == self.midi_event {
                let frames = unsafe { ev.time_frames() };
                self.run_voices(ports, offset, frames as usize);

                let msg = unsafe { ev.contents() };
                let msg_type = midi::message_type(msg[0]);
                if msg_type == midi::MSG_NOTE_ON {
                    if msg[2] > 0 {
                        self.note_on(msg[1], msg[2]);
                    }
                    else {
                        self.note_off(msg[1]);
                    }
                }
                else if msg_type == midi::MSG_NOTE_OFF {
                    self.note_off(msg[1]);
                }

                offset = frames as usize;
            }
        }
        self.run_voices(ports, offset, sample_count);
    }
}

impl Synth {
    fn note_on(&mut self, key: u8, vel: u8) {
        for i in 0 .. 32 {
            if self.voices[i].is_none() {
                self.voices[i] = Some(Voice::new(self, key, vel));
                break;
            }
        }
    }

    fn note_off(&mut self, key: u8) {
        for i in 0 .. 32 {
            if let Some(ref mut voice) = self.voices[i] {
                if voice.key == key {
                    voice.state = State::Release;
                }
            }
        }
    }

    fn run_voices(&mut self, ports: &mut <Synth as lv2::Ported>::Ports, start: usize, end:usize) {
        for s in start .. end {
            ports.output[s] = 0_f32;
        }
        for i in 0 .. 32 {
            let mut set_none = false;
            if let Some(ref mut voice) = self.voices[i] {
                set_none = voice.run(ports.output, start, end);
            }
            if set_none {
                self.voices[i] = None;
            }
        }
    }
}


lv2_descriptor! {
    0 => DESCRIPTOR { "https://github.com/rtbo/lv2-rs/plugins/eg-synth" => Synth }
}
