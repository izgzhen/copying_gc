//! Pointers to Space

use trace::*;

pub trait SpacePtrTrait where Self: Trace {
    fn inner(&self) -> *mut (Trace + 'static);
    fn bit_idx(&self) -> usize;
    fn raw_inner(&self) -> *mut u8;
    fn set_raw_inner(&mut self, p: *mut u8);
}

pub struct SpacePtr<T: Trace + Clone + 'static> {
    inner: *mut T,
    bit_idx: usize,
}

impl<T: Trace + Clone + 'static> SpacePtr<T> {
    pub fn new(ptr: *mut T, idx: usize) -> Self {
        SpacePtr {
           inner: ptr,
           bit_idx: idx,
        }
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.inner }
    }

    pub fn inner(&self) -> *mut T {
        self.inner
    }
}


impl<T: Trace + Clone + 'static> Trace for SpacePtr<T> {
    fn mark(&mut self) { unsafe { (*self.inner).mark(); } }
    fn root(&mut self) { unsafe { (*self.inner).root(); } }
    fn unroot(&mut self) { unsafe { (*self.inner).unroot(); } }
    fn subfields(&mut self)  -> Vec<&mut HeapTrait> {
        unsafe { (*self.inner).subfields() }
    }
}

impl<T: Trace + Clone + 'static> SpacePtrTrait for SpacePtr<T> {
    fn inner(&self) -> *mut (Trace + 'static) {
        self.inner as *mut (Trace + 'static)
    }

    fn bit_idx(&self) -> usize {
        self.bit_idx
    }

    fn raw_inner(&self) -> *mut u8 {
        self.inner as *mut u8
    }

    fn set_raw_inner(&mut self, p: *mut u8) {
        self.inner = p as *mut T;
    }

}

impl<T: Trace + Clone + 'static> Clone for SpacePtr<T> {
    fn clone(&self) -> Self {
        SpacePtr {
            inner: self.inner.clone(),
            bit_idx: self.bit_idx,
        }
    }
}

impl<T: Trace + Clone + 'static> SpacePtr<T> {
    pub fn borrow(&self) -> &T {
        unsafe {
            &*self.inner
        }
    }
}
