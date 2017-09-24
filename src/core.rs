
use ffi::core::*;

use libc::{c_char, c_void};

use std::ffi::CStr;
use std::mem::transmute;


pub struct PluginInstance<'h, T>
where
    T: Plugin<'h>,
{
    pub ports_raw: <T as Ported<'h>>::PortsRaw,
    pub state: T,
}

pub trait Plugin<'h>
where
    Self: Ported<'h>,
{
    fn new(sample_rate: f64, bundle_path: &str) -> Self;
    fn activate(&mut self) {}
    fn run(&mut self, ports: &mut Self::Ports, sample_count: usize);
    fn deactivate(&mut self) {}
}


pub extern "C" fn instantiate<'h, T: Plugin<'h>>(
    _descriptor: *const LV2_Descriptor,
    sample_rate: f64,
    bundle_path: *const c_char,
    _features: *const LV2_Feature,
) -> LV2_Handle {
    let amp_ports = <T as Ported>::new_ports_raw();

    let bundle_path = unsafe { CStr::from_ptr(bundle_path).to_str().unwrap() };

    let instance = Box::new(PluginInstance::<T> {
        ports_raw: amp_ports,
        state: T::new(sample_rate, bundle_path),
    });
    Box::into_raw(instance) as LV2_Handle
}

pub extern "C" fn activate<'h, T: Plugin<'h>>(instance: LV2_Handle) {
    let instance: &mut PluginInstance<T> = unsafe { transmute(instance) };
    instance.state.activate();
}

pub extern "C" fn connect_port<'h, T: Plugin<'h>>(
    instance: LV2_Handle,
    port: u32,
    data_location: *mut c_void,
) {
    let instance: &mut PluginInstance<T> = unsafe { transmute(instance) };
    <T as Ported>::connect_port(
        port as usize,
        data_location as *mut _,
        &mut instance.ports_raw,
    );
}


pub extern "C" fn run<'h, T: 'h + Plugin<'h>>(instance: LV2_Handle, sample_count: u32)
where
    T::PortsRaw: Copy,
{
    let instance: &mut PluginInstance<T> = unsafe { transmute(instance) };

    let sample_count = sample_count as usize;

    let mut ports = <T as Ported>::convert_ports(instance.ports_raw, sample_count);
    instance.state.run(&mut ports, sample_count);
}

pub extern "C" fn deactivate<'h, T: Plugin<'h>>(instance: LV2_Handle) {
    let instance: &mut PluginInstance<T> = unsafe { transmute(instance) };
    instance.state.deactivate();
}

pub extern "C" fn cleanup<'h, T: Plugin<'h>>(instance: LV2_Handle) {
    unsafe {
        let _ = Box::from_raw(instance as *mut PluginInstance<T>);
    }
}


pub unsafe trait Ported<'h> {
    type Ports;
    type PortsRaw;
    fn new_ports_raw() -> Self::PortsRaw;
    fn connect_port(port: usize, data: *mut (), ports_raw: &mut Self::PortsRaw);
    fn convert_ports(ports_raw: Self::PortsRaw, sample_count: usize) -> Self::Ports;
}


pub mod meta {

    use std::ptr;
    use std::mem;
    use std::slice;

    pub unsafe trait Port<'h> {
        type FieldRaw;
        type Field;
        fn new_raw() -> Self::FieldRaw;
        fn cast_raw(data: *mut ()) -> Self::FieldRaw;
        fn convert(raw: Self::FieldRaw, sample_count: usize) -> Self::Field;
    }

    pub enum InputControl {}
    unsafe impl<'h> Port<'h> for InputControl {
        type FieldRaw = *const f32;
        type Field = &'h f32;
        fn new_raw() -> Self::FieldRaw {
            ptr::null()
        }
        fn cast_raw(data: *mut ()) -> Self::FieldRaw {
            data as Self::FieldRaw
        }
        fn convert(raw: Self::FieldRaw, _: usize) -> Self::Field {
            unsafe { mem::transmute(raw) }
        }
    }

    pub enum OutputControl {}
    unsafe impl<'h> Port<'h> for OutputControl {
        type FieldRaw = *mut f32;
        type Field = &'h mut f32;
        fn new_raw() -> Self::FieldRaw {
            ptr::null_mut()
        }
        fn cast_raw(data: *mut ()) -> Self::FieldRaw {
            data as Self::FieldRaw
        }
        fn convert(raw: Self::FieldRaw, _: usize) -> Self::Field {
            unsafe { mem::transmute(raw) }
        }
    }

    pub enum InputAudio {}
    unsafe impl<'h> Port<'h> for InputAudio {
        type FieldRaw = *const f32;
        type Field = &'h [f32];
        fn new_raw() -> Self::FieldRaw {
            ptr::null()
        }
        fn cast_raw(data: *mut ()) -> Self::FieldRaw {
            data as Self::FieldRaw
        }
        fn convert(raw: Self::FieldRaw, sample_count: usize) -> Self::Field {
            unsafe { slice::from_raw_parts(raw, sample_count) }
        }
    }

    pub enum OutputAudio {}
    unsafe impl<'h> Port<'h> for OutputAudio {
        type FieldRaw = *mut f32;
        type Field = &'h mut [f32];
        fn new_raw() -> Self::FieldRaw {
            ptr::null_mut()
        }
        fn cast_raw(data: *mut ()) -> Self::FieldRaw {
            data as Self::FieldRaw
        }
        fn convert(raw: Self::FieldRaw, sample_count: usize) -> Self::Field {
            unsafe { slice::from_raw_parts_mut(raw, sample_count) }
        }
    }

}

#[macro_export]
macro_rules! lv2_descriptor {

    (@desc $DESC:ident { $uri:expr => $Plug:ty }) => {
        static mut $DESC: $crate::ffi::LV2_Descriptor = $crate::ffi::LV2_Descriptor {
            URI: b"\0" as *const u8 as _,
            instantiate: Some($crate::instantiate::<$Plug>),
            connect_port: Some($crate::connect_port::<$Plug>),
            activate: Some($crate::activate::<$Plug>),
            run: Some($crate::run::<$Plug>),
            deactivate: Some($crate::deactivate::<$Plug>),
            cleanup: Some($crate::cleanup::<$Plug>),
            extension_data: None,
        };
    };


    ( $( $idx:expr => $DESC:ident { $uri:expr => $Plug:ty } ),+ ) => {

        $(
            lv2_descriptor!{ @desc $DESC { $uri => $Plug } }
        )+

        #[no_mangle]
        pub unsafe extern "C" fn lv2_descriptor (index: u32) -> *const $crate::ffi::LV2_Descriptor
        {
            match index {
                $(
                    $idx => {
                        $DESC.URI = concat!($uri, "\0").as_ptr() as _;
                        &$DESC
                    },
                )+
                _ => { ptr::null() }
            }
        }

    };

}

#[macro_export]
macro_rules! lv2_ports {
    ( $Plug:ty => { $( $idx:expr => $name:ident : $Meta:ty ),+ } ) => {

        #[derive(Copy, Clone)]
        pub struct PortsRaw<'h> {
            $(
                pub $name: <$Meta as $crate::meta::Port<'h>>::FieldRaw
            ),+
        }

        pub struct Ports<'h> {
            $(
                pub $name: <$Meta as $crate::meta::Port<'h>>::Field
            ),+
        }

        unsafe impl<'h> $crate::Ported<'h> for $Plug {
            type PortsRaw = PortsRaw<'h>;
            type Ports = Ports<'h>;
            fn new_ports_raw() -> Self::PortsRaw {
                Self::PortsRaw {
                    $(
                        $name: <$Meta as $crate::meta::Port<'h>>::new_raw()
                    ),+
                }
            }

            fn connect_port(port: usize, data: *mut (), ports_raw: &mut Self::PortsRaw) {
                match port {
                    $(
                        $idx => { ports_raw.$name = <$Meta as $crate::meta::Port<'h>>::cast_raw(data); }
                    ),+
                    _ => {},
                }
            }

            fn convert_ports(ports_raw: Self::PortsRaw, sample_count: usize) -> Self::Ports {
                Self::Ports {
                    $(
                        $name: <$Meta as $crate::meta::Port<'h>>::convert(ports_raw.$name, sample_count)
                    ),+
                }
            }
        }

    }
}
