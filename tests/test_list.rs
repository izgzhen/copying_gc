extern crate copying_gc;

use copying_gc::*;

#[derive(Clone)]
struct ListObj {
    next: Option<Heap<ListObj>>,
    num: i64,
}

impl Traceable for ListObj {
    fn trace_with<'a, Tracer>(&'a mut self, mut t: Tracer)
            where Tracer: FnMut(&'a mut HeapTrait) {

        match self.next {
            Some(ref mut g) => t(g as &mut HeapTrait),
            None => {},
        }
    }
}

#[test]
fn test_runtime() {
    {
        let p = Root::new(ListObj { next: None, num: 99 });
        let p2 = Root::new(ListObj { next: Some(p.to_heap()), num: 1 });

        match p2.borrow().next {
            Some(ref l) => assert_eq!(l.borrow().num, p.borrow().num),
            None => panic!("None"),
        }
    }

    gc_now();
}
