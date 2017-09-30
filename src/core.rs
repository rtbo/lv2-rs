
use ffi::core::*;

use libc::{c_char, c_void};

use std::ffi::CStr;
use std::mem;
use std::marker;
use std::ptr;


pub trait Plugin<'h>
where
    Self: Ported<'h>,
    Self: marker::Sized,
{
    fn new(sample_rate: f64, bundle_path: &str, features: FeatureList<'h>) -> Option<Self>;
    fn activate(&mut self) {}
    fn run(&mut self, ports: &mut Self::Ports, sample_count: usize);
    fn deactivate(&mut self) {}
}


pub struct PluginInstance<'h, T>
where
    T: Plugin<'h>,
{
    pub ports_raw: <T as Ported<'h>>::PortsRaw,
    pub state: T,
}

pub trait Feature<'h>
{
    fn uri() -> &'static str;
    unsafe fn from_raw(raw: RawFeature<'h>) -> Self;
}

pub struct RawFeature<'h> {
    pub raw: *const LV2_Feature,
    marker: marker::PhantomData<&'h ()>,
}

impl<'h> RawFeature<'h> {
    pub fn uri(&self) -> &'h str {
        unsafe {
            CStr::from_ptr((*self.raw).URI).to_str().unwrap()
        }
    }
    pub fn data(&self) -> &'h () {
        unsafe {
            mem::transmute((*self.raw).data)
        }
    }
}

pub struct FeatureList<'h>
{
    raw: *const *const LV2_Feature,
    marker: marker::PhantomData<&'h ()>,
}

impl<'h> Iterator for FeatureList<'h>
{
    type Item = RawFeature<'h>;
    fn next(&mut self) -> Option<Self::Item> {
        let raw = unsafe { *self.raw };

        if raw.is_null() {
            None
        }
        else {
            unsafe {
                self.raw = self.raw.offset(1);
            }
            Some(RawFeature{ raw: raw, marker: marker::PhantomData })
        }
    }
}


pub extern "C" fn instantiate<'h, T: Plugin<'h>>(
    _descriptor: *const LV2_Descriptor,
    sample_rate: f64,
    bundle_path: *const c_char,
    features: *const *const LV2_Feature,
) -> LV2_Handle {

    let bundle_path = unsafe { CStr::from_ptr(bundle_path).to_str().unwrap() };
    if let Some(state) = T::new(sample_rate, bundle_path,
                                FeatureList{raw: features, marker: marker::PhantomData}) {
        let ports = <T as Ported>::new_ports_raw();
        let instance = Box::new(PluginInstance::<T> {
            ports_raw: ports,
            state: state
        });
        Box::into_raw(instance) as LV2_Handle
    }
    else {
        ptr::null_mut()
    }
}

pub extern "C" fn activate<'h, T: Plugin<'h>>(instance: LV2_Handle) {
    let instance: &mut PluginInstance<T> = unsafe { mem::transmute(instance) };
    instance.state.activate();
}

pub extern "C" fn connect_port<'h, T: Plugin<'h>>(
    instance: LV2_Handle,
    port: u32,
    data_location: *mut c_void,
) {
    let instance: &mut PluginInstance<T> = unsafe { mem::transmute(instance) };
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
    let instance: &mut PluginInstance<T> = unsafe { mem::transmute(instance) };

    let sample_count = sample_count as usize;

    let mut ports = <T as Ported>::convert_ports(instance.ports_raw, sample_count);
    instance.state.run(&mut ports, sample_count);
}

pub extern "C" fn deactivate<'h, T: Plugin<'h>>(instance: LV2_Handle) {
    let instance: &mut PluginInstance<T> = unsafe { mem::transmute(instance) };
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


pub const URI: &'static str = "http://lv2plug.in/ns/lv2core";

pub mod class {
    pub const ALLPASSPLUGIN:      &'static str = "http://lv2plug.in/ns/lv2core#AllpassPlugin";
    pub const AMPLIFIERPLUGIN:    &'static str = "http://lv2plug.in/ns/lv2core#AmplifierPlugin";
    pub const ANALYSERPLUGIN:     &'static str = "http://lv2plug.in/ns/lv2core#AnalyserPlugin";
    pub const AUDIOPORT:          &'static str = "http://lv2plug.in/ns/lv2core#AudioPort";
    pub const BANDPASSPLUGIN:     &'static str = "http://lv2plug.in/ns/lv2core#BandpassPlugin";
    pub const CVPORT:             &'static str = "http://lv2plug.in/ns/lv2core#CVPort";
    pub const CHORUSPLUGIN:       &'static str = "http://lv2plug.in/ns/lv2core#ChorusPlugin";
    pub const COMBPLUGIN:         &'static str = "http://lv2plug.in/ns/lv2core#CombPlugin";
    pub const COMPRESSORPLUGIN:   &'static str = "http://lv2plug.in/ns/lv2core#CompressorPlugin";
    pub const CONSTANTPLUGIN:     &'static str = "http://lv2plug.in/ns/lv2core#ConstantPlugin";
    pub const CONTROLPORT:        &'static str = "http://lv2plug.in/ns/lv2core#ControlPort";
    pub const CONVERTERPLUGIN:    &'static str = "http://lv2plug.in/ns/lv2core#ConverterPlugin";
    pub const DELAYPLUGIN:        &'static str = "http://lv2plug.in/ns/lv2core#DelayPlugin";
    pub const DISTORTIONPLUGIN:   &'static str = "http://lv2plug.in/ns/lv2core#DistortionPlugin";
    pub const DYNAMICSPLUGIN:     &'static str = "http://lv2plug.in/ns/lv2core#DynamicsPlugin";
    pub const EQPLUGIN:           &'static str = "http://lv2plug.in/ns/lv2core#EQPlugin";
    pub const ENVELOPEPLUGIN:     &'static str = "http://lv2plug.in/ns/lv2core#EnvelopePlugin";
    pub const EXPANDERPLUGIN:     &'static str = "http://lv2plug.in/ns/lv2core#ExpanderPlugin";
    pub const EXTENSIONDATA:      &'static str = "http://lv2plug.in/ns/lv2core#ExtensionData";
    pub const FEATURE:            &'static str = "http://lv2plug.in/ns/lv2core#Feature";
    pub const FILTERPLUGIN:       &'static str = "http://lv2plug.in/ns/lv2core#FilterPlugin";
    pub const FLANGERPLUGIN:      &'static str = "http://lv2plug.in/ns/lv2core#FlangerPlugin";
    pub const FUNCTIONPLUGIN:     &'static str = "http://lv2plug.in/ns/lv2core#FunctionPlugin";
    pub const GATEPLUGIN:         &'static str = "http://lv2plug.in/ns/lv2core#GatePlugin";
    pub const GENERATORPLUGIN:    &'static str = "http://lv2plug.in/ns/lv2core#GeneratorPlugin";
    pub const HIGHPASSPLUGIN:     &'static str = "http://lv2plug.in/ns/lv2core#HighpassPlugin";
    pub const INPUTPORT:          &'static str = "http://lv2plug.in/ns/lv2core#InputPort";
    pub const INSTRUMENTPLUGIN:   &'static str = "http://lv2plug.in/ns/lv2core#InstrumentPlugin";
    pub const LIMITERPLUGIN:      &'static str = "http://lv2plug.in/ns/lv2core#LimiterPlugin";
    pub const LOWPASSPLUGIN:      &'static str = "http://lv2plug.in/ns/lv2core#LowpassPlugin";
    pub const MIXERPLUGIN:        &'static str = "http://lv2plug.in/ns/lv2core#MixerPlugin";
    pub const MODULATORPLUGIN:    &'static str = "http://lv2plug.in/ns/lv2core#ModulatorPlugin";
    pub const MULTIEQPLUGIN:      &'static str = "http://lv2plug.in/ns/lv2core#MultiEQPlugin";
    pub const OSCILLATORPLUGIN:   &'static str = "http://lv2plug.in/ns/lv2core#OscillatorPlugin";
    pub const OUTPUTPORT:         &'static str = "http://lv2plug.in/ns/lv2core#OutputPort";
    pub const PARAEQPLUGIN:       &'static str = "http://lv2plug.in/ns/lv2core#ParaEQPlugin";
    pub const PHASERPLUGIN:       &'static str = "http://lv2plug.in/ns/lv2core#PhaserPlugin";
    pub const PITCHPLUGIN:        &'static str = "http://lv2plug.in/ns/lv2core#PitchPlugin";
    pub const PLUGIN:             &'static str = "http://lv2plug.in/ns/lv2core#Plugin";
    pub const PLUGINBASE:         &'static str = "http://lv2plug.in/ns/lv2core#PluginBase";
    pub const POINT:              &'static str = "http://lv2plug.in/ns/lv2core#Point";
    pub const PORT:               &'static str = "http://lv2plug.in/ns/lv2core#Port";
    pub const PORTPROPERTY:       &'static str = "http://lv2plug.in/ns/lv2core#PortProperty";
    pub const RESOURCE:           &'static str = "http://lv2plug.in/ns/lv2core#Resource";
    pub const REVERBPLUGIN:       &'static str = "http://lv2plug.in/ns/lv2core#ReverbPlugin";
    pub const SCALEPOINT:         &'static str = "http://lv2plug.in/ns/lv2core#ScalePoint";
    pub const SIMULATORPLUGIN:    &'static str = "http://lv2plug.in/ns/lv2core#SimulatorPlugin";
    pub const SPATIALPLUGIN:      &'static str = "http://lv2plug.in/ns/lv2core#SpatialPlugin";
    pub const SPECIFICATION:      &'static str = "http://lv2plug.in/ns/lv2core#Specification";
    pub const SPECTRALPLUGIN:     &'static str = "http://lv2plug.in/ns/lv2core#SpectralPlugin";
    pub const UTILITYPLUGIN:      &'static str = "http://lv2plug.in/ns/lv2core#UtilityPlugin";
    pub const WAVESHAPERPLUGIN:   &'static str = "http://lv2plug.in/ns/lv2core#WaveshaperPlugin";
}
pub mod prop {
    pub const APPLIESTO:          &'static str = "http://lv2plug.in/ns/lv2core#appliesTo";
    pub const BINARY:             &'static str = "http://lv2plug.in/ns/lv2core#binary";
    pub const DEFAULT:            &'static str = "http://lv2plug.in/ns/lv2core#default";
    pub const DESIGNATION:        &'static str = "http://lv2plug.in/ns/lv2core#designation";
    pub const DOCUMENTATION:      &'static str = "http://lv2plug.in/ns/lv2core#documentation";
    pub const EXTENSIONDATA:      &'static str = "http://lv2plug.in/ns/lv2core#extensionData";
    pub const FREEWHEELING:       &'static str = "http://lv2plug.in/ns/lv2core#freeWheeling";
    pub const INDEX:              &'static str = "http://lv2plug.in/ns/lv2core#index";
    pub const INTEGER:            &'static str = "http://lv2plug.in/ns/lv2core#integer";
    pub const LATENCY:            &'static str = "http://lv2plug.in/ns/lv2core#latency";
    pub const MAXIMUM:            &'static str = "http://lv2plug.in/ns/lv2core#maximum";
    pub const MICROVERSION:       &'static str = "http://lv2plug.in/ns/lv2core#microVersion";
    pub const MINIMUM:            &'static str = "http://lv2plug.in/ns/lv2core#minimum";
    pub const MINORVERSION:       &'static str = "http://lv2plug.in/ns/lv2core#minorVersion";
    pub const NAME:               &'static str = "http://lv2plug.in/ns/lv2core#name";
    pub const OPTIONALFEATURE:    &'static str = "http://lv2plug.in/ns/lv2core#optionalFeature";
    pub const PORT:               &'static str = "http://lv2plug.in/ns/lv2core#port";
    pub const PORTPROPERTY:       &'static str = "http://lv2plug.in/ns/lv2core#portProperty";
    pub const PROJECT:            &'static str = "http://lv2plug.in/ns/lv2core#project";
    pub const PROTOTYPE:          &'static str = "http://lv2plug.in/ns/lv2core#prototype";
    pub const REQUIREDFEATURE:    &'static str = "http://lv2plug.in/ns/lv2core#requiredFeature";
    pub const SCALEPOINT:         &'static str = "http://lv2plug.in/ns/lv2core#scalePoint";
    pub const SYMBOL:             &'static str = "http://lv2plug.in/ns/lv2core#symbol";
}
pub mod inst {
    pub const CONNECTIONOPTIONAL: &'static str = "http://lv2plug.in/ns/lv2core#connectionOptional";
    pub const CONTROL:            &'static str = "http://lv2plug.in/ns/lv2core#control";
    pub const ENUMERATION:        &'static str = "http://lv2plug.in/ns/lv2core#enumeration";
    pub const HARDRTCAPABLE:      &'static str = "http://lv2plug.in/ns/lv2core#hardRTCapable";
    pub const INPLACEBROKEN:      &'static str = "http://lv2plug.in/ns/lv2core#inPlaceBroken";
    pub const ISLIVE:             &'static str = "http://lv2plug.in/ns/lv2core#isLive";
    pub const REPORTSLATENCY:     &'static str = "http://lv2plug.in/ns/lv2core#reportsLatency";
    pub const SAMPLERATE:         &'static str = "http://lv2plug.in/ns/lv2core#sampleRate";
    pub const TOGGLED:            &'static str = "http://lv2plug.in/ns/lv2core#toggled";
}
