//! Trace trait

use heap::*;

use std::cell::RefCell;

pub trait Trace {
    /// Mark the subfields (recursively if necessary)
    fn mark(&mut self);

    /// Called when unrooted from stack
    fn unroot(&mut self);

    /// Called when rooted to stack
    fn root(&mut self);

    /// Traverse the sub-fields
    fn subfields(&mut self) -> Vec<&mut SpacePtrTrait>;
}

impl Traceable for i64 {
    fn trace_with<'a, Tracer>(&'a mut self, _: Tracer) where Tracer: FnMut(&'a mut SpacePtrTrait) { }
}

pub trait Traceable {
    fn trace_with<'a, Tracer>(&'a mut self, Tracer) where Tracer: FnMut(&'a mut SpacePtrTrait);
}

impl<T: Traceable> Trace for T {

    fn mark(&mut self) {
        self.trace_with(|field| field.mark());
    }

    fn root(&mut self) {
        self.trace_with(|field| field.root());
    }

    fn unroot(&mut self) {
        self.trace_with(|field| field.unroot());
    }

    fn subfields(&mut self) -> Vec<&mut SpacePtrTrait> {
        let ret = RefCell::new(vec![]);

        self.trace_with(|field| {
            ret.borrow_mut().push(field);
        });

        ret.into_inner()
    }
}
