//! Homebrew BitMap

use alloc::heap;
use std::ops::{Shr, Shl, Rem, BitOrAssign, BitAnd, BitAndAssign, Not};

pub struct BitMap {
    /// 16 bits (or u256) unit, but addressed by u16 anyway
    inner: *mut u16,
    /// Size in bits
    size: usize,
}

impl BitMap {
    pub fn new(size: usize) -> BitMap {
        debug_assert!(size > 0);

        let u256_num = (size as i32 - 1).shr(3) + 1;
        let u8_num = u256_num * 16;

        BitMap {
            inner: unsafe { heap::allocate(u8_num as usize, 1) as *mut u16 },
            size: size,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get(&self, idx: usize) -> bool {
        debug_assert!(idx < self.size);

        let u256_idx: usize = idx.shr(3);
        let inner_idx = idx.rem(8);

        unsafe {
            let u256_start = self.inner.offset(u256_idx.shl(4) as isize);
            
            if inner_idx < 4 {
                (*u256_start).bitand(1.shl(inner_idx) as u16) != 0
            } else {
                let u256_start = u256_start.offset(1);
                (*u256_start).bitand(1.shl(inner_idx) as u16) != 0
            }
        }
    }

    pub fn set(&mut self, idx: usize) {
        debug_assert!(idx < self.size);

        let u256_idx: usize = idx.shr(3);
        let inner_idx = idx.rem(8);

        unsafe {
            let u256_start = self.inner.offset(u256_idx.shl(4) as isize);
            
            if inner_idx < 4 {
                (*u256_start).bitor_assign(1.shl(inner_idx) as u16);
            } else {
                let u256_start = u256_start.offset(1);
                (*u256_start).bitor_assign(1.shl(inner_idx) as u16);
            }
        }
    }

    pub fn unset(&mut self, idx: usize) {
        debug_assert!(idx < self.size);

        let u256_idx: usize = idx.shr(3);
        let inner_idx = idx.rem(8);

        unsafe {

            let u256_start = self.inner.offset(u256_idx.shl(4) as isize);
            
            let x: u16 = 1.shl(inner_idx) as u16;

            if inner_idx < 4 {
                (*u256_start).bitand_assign(x.not());
            } else {
                let u256_start = u256_start.offset(1);
                (*u256_start).bitand_assign(x.not());
            }
        }
    }
}


#[test]
fn test_bitmap() {
    let mut m = BitMap::new(16);

    for i in 0..16 {
        m.set(i);

        assert_eq!(m.get(i), true);

        for j in 0..16 {
            if j != i {
                assert_eq!(m.get(j), false);
            }
        }

        m.unset(i);
    }
}
