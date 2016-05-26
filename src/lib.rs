#![feature(alloc, heap_api)]
#![allow(dead_code)]

extern crate alloc;

mod bitmap;
mod space;
mod runtime;
mod trace;
mod ptr;
mod space_ptr;

pub use trace::{Traceable, HeapTrait};
pub use ptr::{Root, gc_now, Heap};

