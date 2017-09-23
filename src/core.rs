
#![allow(non_upper_case_globals)]

//use ffi::core::*;

pub struct PluginInstance<'h, T>
    where T : Plugin<'h>
{
    pub ports_raw: <T as Ported<'h>>::PortsRaw,
    pub state: T,
}

pub trait Plugin<'h> where Self : Ported<'h>
{
    fn new (sample_rate: f64, bundle_path: &str) -> Self;
    fn activate(&mut self) {}
    fn run(&mut self, ports: &'h mut Self::Ports, sample_count: usize);
    fn deactivate(&mut self) {}
}

pub unsafe trait Ported<'h> {
    type Ports;
    type PortsRaw;
    fn new_ports_raw() -> Self::PortsRaw;
    fn connect_port(port: usize, data: *mut (), ports_raw: &mut Self::PortsRaw);
    fn convert_ports(ports_raw: Self::PortsRaw, sample_count: usize) -> Self::Ports;
}


pub mod meta {

    pub trait Port {
        type Field;
    }

    pub enum InputControl {}
    impl Port for InputControl {
        type Field = *const f32;
    }

    pub enum OutputControl {}
    impl Port for OutputControl {
        type Field = *mut f32;
    }

    pub enum InputAudio {}
    impl Port for InputAudio {
        type Field = *const f32;
    }

    pub enum OutputAudio {}
    impl Port for OutputAudio {
        type Field = *mut f32;
    }

}
