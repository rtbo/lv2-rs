
use ::atom::*;

use std::mem;

/// pad to 64 bits
pub fn pad_size(size: usize) -> usize {
    (size + 7) & (!7)
}

pub unsafe fn sequence_begin(body: &SequenceBody) -> *const Event {
    let ptr = body as *const SequenceBody;
    ptr.offset(1) as *const Event
}

pub unsafe fn sequence_is_end(body: &SequenceBody, size: usize, ev: *const Event) -> bool {
    let ptr = body as *const SequenceBody as *const u8;
	ev as *const u8 >= ptr.offset(size as isize)
}

pub unsafe fn sequence_next(ev: *const Event) -> *const Event {
	let ptr = ev as *const u8;
	ptr.offset((mem::size_of::<Event>() + pad_size((*ev).size())) as isize) as *const Event
}
