//! Rooted pointer

use space::*;
use trace::*;
use runtime::*;
use heap::*;

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

pub struct Root<T: Trace + Clone + 'static>(*mut Rooted<T>);

impl<T: Trace + Clone + 'static> Root<T> {
    pub fn new(t: T) -> Root<T> {
        RUNTIME.with(|_rt| {
            let mut rt = _rt.borrow_mut();

            rt.cons(t)
        })
    }

    fn from_rooted(ptr: *mut Rooted<T>) -> Root<T> {
        Root(ptr)
    }

    pub fn borrow(&self) -> &T {
        unsafe { (*self.0).borrow() }
    }

    pub fn to_space_ptr(&self) -> SpacePtr<T> {
        unsafe {
            (*self.0).space_ptr.clone()
        }
    }
}


impl<T: Trace + Clone + 'static> Clone for Root<T> {
    fn clone(&self) -> Self {
        Root(self.0)
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
    fn subfields(&mut self)  -> Vec<&mut SpacePtrTrait> {
        self.space_ptr.subfields()
    }
}
