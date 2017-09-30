
use urid;

use std::mem;
use std::slice;
use std::str;
use std::ffi::{CStr};

mod util;

pub const URI: &'static str = "http://lv2plug.in/ns/ext/atom";

pub mod class {
    pub const ATOM: &'static str = "http://lv2plug.in/ns/ext/atom#Atom";
    pub const ATOMPORT: &'static str = "http://lv2plug.in/ns/ext/atom#AtomPort";
    pub const BLANK: &'static str = "http://lv2plug.in/ns/ext/atom#Blank";
    pub const BOOL: &'static str = "http://lv2plug.in/ns/ext/atom#Bool";
    pub const CHUNK: &'static str = "http://lv2plug.in/ns/ext/atom#Chunk";
    pub const DOUBLE: &'static str = "http://lv2plug.in/ns/ext/atom#Double";
    pub const EVENT: &'static str = "http://lv2plug.in/ns/ext/atom#Event";
    pub const FLOAT: &'static str = "http://lv2plug.in/ns/ext/atom#Float";
    pub const INT: &'static str = "http://lv2plug.in/ns/ext/atom#Int";
    pub const LITERAL: &'static str = "http://lv2plug.in/ns/ext/atom#Literal";
    pub const LONG: &'static str = "http://lv2plug.in/ns/ext/atom#Long";
    pub const NUMBER: &'static str = "http://lv2plug.in/ns/ext/atom#Number";
    pub const OBJECT: &'static str = "http://lv2plug.in/ns/ext/atom#Object";
    pub const PATH: &'static str = "http://lv2plug.in/ns/ext/atom#Path";
    pub const PROPERTY: &'static str = "http://lv2plug.in/ns/ext/atom#Property";
    pub const RESOURCE: &'static str = "http://lv2plug.in/ns/ext/atom#Resource";
    pub const SEQUENCE: &'static str = "http://lv2plug.in/ns/ext/atom#Sequence";
    pub const SOUND: &'static str = "http://lv2plug.in/ns/ext/atom#Sound";
    pub const STRING: &'static str = "http://lv2plug.in/ns/ext/atom#String";
    pub const TUPLE: &'static str = "http://lv2plug.in/ns/ext/atom#Tuple";
    pub const URI: &'static str = "http://lv2plug.in/ns/ext/atom#URI";
    pub const URID: &'static str = "http://lv2plug.in/ns/ext/atom#URID";
    pub const VECTOR: &'static str = "http://lv2plug.in/ns/ext/atom#Vector";
}

pub mod prop {
    pub const BEATTIME: &'static str = "http://lv2plug.in/ns/ext/atom#beatTime";
    pub const BUFFERTYPE: &'static str = "http://lv2plug.in/ns/ext/atom#bufferType";
    pub const CHILDTYPE: &'static str = "http://lv2plug.in/ns/ext/atom#childType";
    pub const FRAMETIME: &'static str = "http://lv2plug.in/ns/ext/atom#frameTime";
    pub const SUPPORTS: &'static str = "http://lv2plug.in/ns/ext/atom#supports";
    pub const TIMEUNIT: &'static str = "http://lv2plug.in/ns/ext/atom#timeUnit";
}

pub mod inst {
    pub const ATOMTRANSFER: &'static str = "http://lv2plug.in/ns/ext/atom#atomTransfer";
    pub const EVENTTRANSFER: &'static str = "http://lv2plug.in/ns/ext/atom#eventTransfer";
}


pub unsafe fn contents<'a, T: Atom>(atom: &'a T) -> &'a [u8] {
    let ptr = atom as *const T as *const u8;
    let ptr = ptr.offset(mem::size_of::<T>() as isize);
    debug_assert!(mem::size_of::<T>() >= mem::size_of::<Header>());
    slice::from_raw_parts(ptr, atom.size() - (mem::size_of::<T>() - mem::size_of::<Header>()))
}

pub unsafe fn contents_mut<'a, T: Atom>(atom: &'a mut T) -> &'a mut [u8] {
    let ptr = atom as *mut T as *mut u8;
    let ptr = ptr.offset(mem::size_of::<T>() as isize);
    debug_assert!(mem::size_of::<T>() >= mem::size_of::<Header>());
    slice::from_raw_parts_mut(ptr, atom.size() - (mem::size_of::<T>() - mem::size_of::<Header>()))
}

pub trait Atom {
    fn type_uri() -> &'static str;
    fn type_urid(&self) -> urid::URID;
    fn size(&self) -> usize;
    fn header(&self) -> Header;
}

pub trait Value<T>
where
    Self: Atom,
{
    fn value(&self) -> T;
}

pub trait Contents<T>
where
    Self: Atom,
    T: ?Sized,
{
    unsafe fn contents(&self) -> &T;
}

#[derive(Copy, Clone)]
pub struct Header {
    size: u32,
    type_urid: urid::URID,
}

impl Header {
    pub fn size(&self) -> usize {
        self.size as usize
    }
    pub fn type_urid(&self) -> urid::URID {
        self.type_urid
    }
    pub unsafe fn contents(&self) -> &[u8] {
        let ptr = self as *const Header;
        slice::from_raw_parts(ptr.offset(1) as *const u8, self.size())
    }
    pub unsafe fn contents_mut(&mut self) -> &mut [u8] {
        let ptr = self as *mut Header;
        slice::from_raw_parts_mut(ptr.offset(1) as *mut u8, self.size())
    }
}

#[derive(Copy, Clone, Atom)]
#[AtomURI = "class::INT"]
pub struct Int {
    header: Header,
    body: i32,
}

impl Value<i32> for Int {
    fn value(&self) -> i32 {
        self.body
    }
}



#[derive(Copy, Clone, Atom)]
#[AtomURI = "class::LONG"]
pub struct Long {
    header: Header,
    body: i64,
}

impl Value<i64> for Long {
    fn value(&self) -> i64 {
        self.body
    }
}

#[derive(Copy, Clone, Atom)]
#[AtomURI = "class::FLOAT"]
pub struct Float {
    header: Header,
    body: f32,
}

impl Value<f32> for Float {
    fn value(&self) -> f32 {
        self.body
    }
}

#[derive(Copy, Clone, Atom)]
#[AtomURI = "class::DOUBLE"]
pub struct Double {
    header: Header,
    body: f64,
}

impl Value<f64> for Double {
    fn value(&self) -> f64 {
        self.body
    }
}

#[derive(Copy, Clone, Atom)]
#[AtomURI = "class::BOOL"]
pub struct Bool {
    header: Header,
    body: i32,
}

impl Value<bool> for Bool {
    fn value(&self) -> bool {
        self.body != 0
    }
}

#[derive(Copy, Clone, Atom)]
#[AtomURI = "class::URID"]
pub struct URID {
    header: Header,
    body: urid::URID,
}

impl Value<urid::URID> for URID {
    fn value(&self) -> urid::URID {
        self.body
    }
}


#[derive(Atom)]
#[AtomURI = "class::STRING"]
pub struct String {
    header: Header,
    // content
}

impl Contents<str> for String {
    unsafe fn contents(&self) -> &str {
        let ptr = self as *const String as *const i8;
        let ptr = ptr.offset(mem::size_of::<String>() as isize);
        let cstr = CStr::from_ptr(ptr);
        cstr.to_str().unwrap()
    }
}

#[allow(dead_code)]
pub struct LiteralBody {
    datatype: urid::URID,
    lang: urid::URID,
    // content
}

#[derive(Atom)]
#[AtomURI = "class::LITERAL"]
#[allow(dead_code)]
pub struct Literal {
    header: Header,
    body: LiteralBody,
}

impl Contents<str> for Literal {
    unsafe fn contents(&self) -> &str {
        let ptr = self as *const Literal as *const i8;
        let ptr = ptr.offset(mem::size_of::<Literal>() as isize);
        let cstr = CStr::from_ptr(ptr);
        cstr.to_str().unwrap()
    }
}

#[derive(Atom)]
#[AtomURI = "class::TUPLE"]
pub struct Tuple {
    header: Header,
    // content
}

#[allow(dead_code)]
pub struct VectorBody {
    child_size: u32,
    child_type: u32,
    // content
}

#[derive(Atom)]
#[AtomURI = "class::VECTOR"]
#[allow(dead_code)]
pub struct Vector {
    header: Header,
    body: VectorBody,
}

#[allow(dead_code)]
pub struct PropertyBody {
    key: urid::URID,
    context: urid::URID,
    value: Header,
    // value body
}

#[derive(Atom)]
#[AtomURI = "class::PROPERTY"]
#[allow(dead_code)]
pub struct Property {
    header: Header,
    body: PropertyBody,
}

#[allow(dead_code)]
pub struct ObjectBody {
    id: urid::URID,
    otype: urid::URID,
    // content
}

#[derive(Atom)]
#[AtomURI = "class::OBJECT"]
#[allow(dead_code)]
pub struct Object {
    header: Header,
    body: ObjectBody,
}

pub struct Event {
    frames: i64,
    body: Header,
    // content
}

impl Event {
    pub unsafe fn time_frames(&self) -> i64 {
        self.frames
    }
    pub unsafe fn time_beats(&self) -> f64 {
        mem::transmute(self.frames)
    }
    pub fn type_urid(&self) -> urid::URID {
        self.body.type_urid
    }
    pub fn size(&self) -> usize {
        self.body.size as usize
    }
    pub unsafe fn contents(&self) -> &[u8] {
        slice::from_raw_parts(
            (self as *const Event as *const u8).offset(mem::size_of::<Event>() as isize),
            self.body.size as usize
        )
    }
}

#[allow(dead_code)]
pub struct SequenceBody {
    unit: urid::URID,
    pad: u32,
}

#[derive(Atom)]
#[AtomURI = "class::SEQUENCE"]
#[allow(dead_code)]
pub struct Sequence {
    header: Header,
    body: SequenceBody,
}

impl Sequence {
    pub fn iter<'a>(&'a self) -> SequenceIter<'a> {
        SequenceIter {
            seq: self,
            event: unsafe { util::sequence_begin(&self.body) }
        }
    }
}

pub struct SequenceIter<'a> {
    seq: &'a Sequence,
    event: *const Event,
}

impl<'a> Iterator for SequenceIter<'a>
{
    type Item = &'a Event;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if util::sequence_is_end(&self.seq.body, self.seq.size(), self.event) {
                None
            }
            else {
                let ev = self.event;
                self.event = util::sequence_next(self.event);
                Some(mem::transmute(ev))
            }
        }
    }
}

pub mod meta {
    use ::core::meta;
    use ::atom::Sequence;
    use std::mem;
    use std::ptr;

    pub enum InputSequence {}
    unsafe impl<'h> meta::Port<'h> for InputSequence {
        type FieldRaw = *const Sequence;
        type Field = &'h Sequence;
        fn new_raw() -> Self::FieldRaw {
            ptr::null()
        }
        fn cast_raw(data: *mut ()) -> Self::FieldRaw {
            data as Self::FieldRaw
        }
        fn convert(raw: Self::FieldRaw, _sample_count: usize) -> Self::Field {
            unsafe { mem::transmute(raw) }
        }
    }
}
