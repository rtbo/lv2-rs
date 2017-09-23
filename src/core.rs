
use ffi::core::*;

use libc::{c_char, c_void};

use std::ffi::CStr;
use std::mem::transmute;


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
    fn run(&mut self, ports: &mut Self::Ports, sample_count: usize);
    fn deactivate(&mut self) {}
}

pub unsafe trait Ported<'h> {
    type Ports;
    type PortsRaw;
    fn new_ports_raw() -> Self::PortsRaw;
    fn connect_port(port: usize, data: *mut (), ports_raw: &mut Self::PortsRaw);
    fn convert_ports(ports_raw: Self::PortsRaw, sample_count: usize) -> Self::Ports;
}


pub extern fn instantiate<'h, T : Plugin<'h>> (_descriptor: *const LV2_Descriptor,
                                           sample_rate: f64,
                                           bundle_path: *const c_char,
                                           _features: *const LV2_Feature)
        -> LV2_Handle
{
    let amp_ports = <T as Ported>::new_ports_raw();

    let bundle_path = unsafe { CStr::from_ptr(bundle_path).to_str().unwrap() };

    let instance = Box::new(
        PluginInstance::<T> {
            ports_raw: amp_ports,
            state: T::new(sample_rate, bundle_path),
        }
    );
    Box::into_raw(instance) as LV2_Handle
}

pub extern fn connect_port<'h, T : Plugin<'h>> (instance: LV2_Handle,
                        port: u32,
                        data_location: *mut c_void)
{
    let instance: &mut PluginInstance<T> = unsafe { transmute(instance) };
    <T as Ported>::connect_port(port as usize,
                                       data_location as *mut _,
                                       &mut instance.ports_raw);
}


pub extern fn run<'h, T : 'h + Plugin<'h>> (instance: LV2_Handle, sample_count: u32)
        where T::PortsRaw : Copy
{
    let instance: &mut PluginInstance<T> = unsafe { transmute(instance) };

    let sample_count = sample_count as usize;

    let mut ports = <T as Ported>::convert_ports(instance.ports_raw, sample_count);
    instance.state.run(&mut ports, sample_count);
}


pub extern fn cleanup<'h, T : Plugin<'h>> (instance: LV2_Handle)
{
    unsafe { let _ = Box::from_raw(instance as *mut PluginInstance<T>); }
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
