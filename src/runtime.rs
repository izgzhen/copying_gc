//! Runtime

use space::*;
use trace::*;
use ptr::*;

const DEFAULT_SPACE_SIZE: usize = 1024;

pub struct Runtime {
    space: Space,
    allocated: Vec<Box<RootedTrait>>,
}


impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            space: Space::new(DEFAULT_SPACE_SIZE),
            allocated: vec![],
        }
    }

    pub fn cons<T: Trace + Clone + 'static>(&mut self, t: T) -> Root<T> {
        loop {
            if let Some(ptr) = self.space.allocate(t.clone()) {

                let mut rooted_box = Box::new(Rooted::new(ptr));

                // HACK
                let rooted_ptr = &mut *rooted_box as *mut _;

                self.allocated.push(rooted_box);

                return Rooted::into_root(rooted_ptr);
            } else {
                self.gc();
            }
        }
    }

    pub fn gc(&mut self) {
        let mut new_space = Space::new(DEFAULT_SPACE_SIZE);
        let mut new_allocated: Vec<Box<RootedTrait>> = vec![];

        for rooted in &mut self.allocated {
            let rooted = &mut **rooted;

            if rooted.rooted() {
                new_space.realloc(rooted);
                unsafe { new_allocated.push(Box::from_raw(rooted)); }
            }
        }

        for rooted in &mut new_allocated {
            for sub in (&mut *rooted.space_ptr()).subfields() {
                self.space.forward(sub);
            }
        }

        println!("new_allocated {:?}", new_allocated.len());

        self.allocated = new_allocated;
        self.space = new_space;
    }
}