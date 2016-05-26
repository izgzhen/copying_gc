//! Pointers

use space::*;
use trace::*;
use runtime::*;
use space_ptr::*;

use std::cell::RefCell;

thread_local!(static RUNTIME: RefCell<Runtime> = RefCell::new(Runtime::new()));

pub trait RootedTrait {
    fn rooted(&self) -> bool;
    fn space_ptr(&mut self) -> &mut SpacePtrTrait;
    fn realloc_to(&mut self, &mut Space);
}

pub fn gc_now() {
    RUNTIME.with(|_rt| {
        _rt.borrow_mut().gc()
    })
}

/// On-stack pointers
pub struct Root<T: Trace + Clone + 'static>(*mut Rooted<T>);

impl<T: Traceable> Root<T> {
    pub fn new(t: T) -> Root<T> {
        RUNTIME.with(|_rt| {
            let mut rt = _rt.borrow_mut();

            rt.cons(t)
        })
    }

    pub fn borrow(&self) -> &T {
        unsafe { (*self.0).borrow() }
    }

    pub fn to_heap(&self) -> Heap<T> {
        unsafe { Heap((*self.0).space_ptr.clone()) }
    }
}

impl<T: Trace + Clone + 'static> Drop for Root<T> {
    fn drop(&mut self) {
        println!("Dropping Root");
        unsafe { (&mut *self.0).rooted = false; }
    }
}

pub struct Rooted<T: Trace + Clone + 'static> {
    rooted: bool,
    space_ptr: SpacePtr<T>,
}

impl<T: Trace + Clone + 'static> Rooted<T> {
    pub fn new(ptr: SpacePtr<T>) -> Rooted<T> {
        Rooted {
            rooted: true,
            space_ptr: ptr,
        }
    }

    pub fn borrow(&self) -> &T {
        self.space_ptr.borrow()
    }

    pub fn into_root(ptr: *mut Self) -> Root<T> {
        Root(ptr)
    }
}

impl<T: Trace + Clone + 'static> RootedTrait for Rooted<T> {
    fn rooted(&self) -> bool { self.rooted }

    fn space_ptr(&mut self) -> &mut SpacePtrTrait {
        &mut self.space_ptr
    }

    fn realloc_to(&mut self, space: &mut Space) {
        unsafe {
            let space_ptr = space.allocate((*self.space_ptr.inner()).clone()).unwrap();
            space.set_forward(&self.space_ptr);
            *(self.space_ptr.inner() as *mut (*mut T)) = space_ptr.inner();
            self.space_ptr =  space_ptr;
        }
    }
}

impl<T: Trace + Clone + 'static> Trace for Rooted<T> {
    fn mark(&mut self) { self.space_ptr.mark(); }
    fn root(&mut self) { self.space_ptr.root(); }
    fn unroot(&mut self) { self.space_ptr.unroot(); }
    fn subfields(&mut self)  -> Vec<&mut HeapTrait> {
        self.space_ptr.subfields()
    }
}

/// On-heap pointers
#[derive(Clone)]
pub struct Heap<T: Traceable>(SpacePtr<T>);

impl<T: Traceable> Heap<T> {
    pub fn borrow(&self) -> &T {
        self.0.borrow()
    }
}

impl<T: Traceable> SpacePtrTrait for Heap<T> {
    fn inner(&self) -> *mut (Trace + 'static) { self.0.inner() }
    fn bit_idx(&self) -> usize { self.0.bit_idx() }
    fn raw_inner(&self) -> *mut u8 { self.0.raw_inner() }
    fn set_raw_inner(&mut self, p: *mut u8) { self.0.set_raw_inner(p) }
}

impl<T: Traceable> Trace for Heap<T> {
    fn mark(&mut self) { self.0.mark(); }
    fn root(&mut self) { self.0.root(); }
    fn unroot(&mut self) { self.0.unroot(); }
    fn subfields(&mut self)  -> Vec<&mut HeapTrait> {
        self.0.subfields()
    }
}

impl<T: Traceable> HeapTrait for Heap<T> { }

