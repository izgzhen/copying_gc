//! Space manager

use alloc::heap as alloc_heap;
use std::ops::{Shl, Shr};
use std::mem;

use bitmap::BitMap;
use trace::*;
use root::*;
use heap::*;

pub struct Space {
    /// How many double words
    size: usize,
    /// Pointer to the first double word
    start_ptr: *mut u64,
    /// Pointer to the first free double word
    free_ptr: *mut u64,
    /// Pointer to the first double word beyond range
    end_ptr: *mut u64,
    /// Mark bits
    mark_bits: BitMap,
    /// Forwarded bits
    forward_bits: BitMap,
    /// BitMap index of first free double word
    free_idx: usize,
}

impl Space {
    pub fn new(size: usize) -> Space {
        unsafe {
            let p = alloc_heap::allocate(size.shl(3), mem::size_of::<u64>()) as *mut u64;
            Space {
                size: size,
                start_ptr: p.clone(),
                end_ptr: p.offset(size.shl(3) as isize),
                free_ptr: p,
                mark_bits: BitMap::new(size),
                forward_bits: BitMap::new(size),
                free_idx: 0,
            }
        }
    }

    pub fn allocate<T: Trace + Clone + Sized + 'static>(&mut self, t: T) -> Option<SpacePtr<T>> {
        unsafe {
            let size_in_bytes = mem::size_of::<T>() as isize;
            let size_in_dws: isize = ((size_in_bytes as i32 - 1).shr(3) + 1) as isize;
            let size_aligned = size_in_dws.shl(3);
            let next_free = self.free_ptr.offset(size_aligned);
            if  next_free > self.end_ptr {
                None
            } else {
                *(self.free_ptr as *mut T) = t;

                let ret = SpacePtr::new(self.free_ptr as *mut T, self.free_idx);

                self.free_ptr = next_free;
                self.free_idx += size_in_dws as usize;
                Some(ret)
            }
        }
    }

    pub fn realloc(&mut self, p: &mut RootedTrait) {
        p.realloc_to(self)
    }

    pub fn set_mark<T: Trace + Clone + ?Sized + 'static>(&mut self, p: &SpacePtr<T>) {
        self.mark_bits.set(p.bit_idx());
    }

    pub fn unset_mark<T: Trace + Clone + ?Sized + 'static>(&mut self, p: &SpacePtr<T>) {
        self.mark_bits.unset(p.bit_idx());
    }

    pub fn marked(&mut self, p: &SpacePtrTrait) -> bool {
        self.mark_bits.get(p.bit_idx())
    }

    pub fn set_forward<T: Trace + Clone + ?Sized + 'static>(&mut self, p: &SpacePtr<T>) {
        self.forward_bits.set(p.bit_idx());
    }

    pub fn forward(&mut self, ptr: &mut SpacePtrTrait) {
        let idx = ptr.bit_idx().clone();
        if self.forward_bits.get(idx) {
            unsafe {
                let forwarded = *(ptr.raw_inner() as *mut (*mut u8));
                ptr.set_raw_inner(forwarded);
            }
        }
    }
}

impl Drop for Space {
    fn drop(&mut self) {
        unsafe {
            alloc_heap::deallocate(self.start_ptr as *mut u8,
                             self.size.shl(3),
                             mem::size_of::<u64>())
        }
    }
}

#[test]
fn test_space() {
    let mut space = Space::new(1024);

    for i in 0..500 {
        let p: SpacePtr<i64> = space.allocate(i as i64).unwrap();

        assert_eq!(*p.borrow(), i);
    }
}
