#![feature(alloc, heap_api)]
#![allow(dead_code)]

extern crate alloc;

mod bitmap;
mod space;
mod runtime;
mod trace;
mod root;
mod heap;

pub use trace::Traceable;
pub use root::{Root, gc_now};
pub use heap::{SpacePtr, SpacePtrTrait};
