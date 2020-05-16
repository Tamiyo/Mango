use crate::vm::managed::Managed;
use crate::vm::managed::Allocation;
use crate::vm::managed::Header;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::ptr::NonNull;

thread_local!(static HEAP: RefCell<Heap> = RefCell::new(Heap::new()));

pub trait Trace {
    fn trace(&self);
}

impl<T: Trace> Trace for RefCell<T> {
    fn trace(&self) {
        self.borrow().trace();
    }
}

impl<T: Trace> Trace for Vec<T> {
    fn trace(&self) {
        for e in self {
            e.trace();
        }
    }
}

impl<K: Eq + Hash, T: Trace> Trace for HashMap<K, T> {
    fn trace(&self) {
        for val in self.values() {
            val.trace();
        }
    }
}

struct Heap {
    objects: Vec<Box<Allocation<dyn Trace>>>,
    bytes_allocated: usize,
    threshold: usize,
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            objects: vec![],
            bytes_allocated: 0,
            threshold: 100,
        }
    }

    fn allocate<T: 'static + Trace>(&mut self, data: T) -> NonNull<Allocation<T>> {
        let mut alloc = Box::new(Allocation {
            header: Header::default(),
            data,
        });

        let ptr = unsafe { NonNull::new_unchecked(&mut *alloc) };
        self.objects.push(alloc);
        ptr
    }

    pub fn manage<T: 'static + Trace>(&mut self, data: T) -> Managed<T> {
        Managed {
            ptr: self.allocate(data),
        }
    }
}

pub fn manage<T: 'static + Trace>(data: T) -> Managed<T> {
    HEAP.with(|heap| heap.borrow_mut().manage(data))
}
