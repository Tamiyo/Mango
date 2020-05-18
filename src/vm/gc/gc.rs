/// Garbage Collection for Mango
///
/// Uses a somewhat naive Mark-Sweep algorithm to reclaim memory during program execution.
///
/// This code is shamelessly copied from Darklox's implementation of Lox:
/// https://github.com/Darksecond/lox/tree/master/lox-vm/src/bettergc
///
/// I would like to revisit this section of the codebase and redesign it in my own image
///  when I have a stronger understanding of Rust and Garbage Collection theory.
use crate::vm::gc::managed::Gc;
use crate::vm::gc::managed::Heap;
use crate::vm::gc::managed::Root;
use crate::vm::gc::managed::Trace;
use crate::vm::gc::managed::UniqueRoot;
use std::cell::RefCell;

struct GcStats {
    bytes_allocated: usize,
    threshold: usize,
}

thread_local!(static STATS: RefCell<GcStats> = RefCell::new(GcStats {
    bytes_allocated: 0,
    threshold: 100,
}));

thread_local!(static HEAP: RefCell<Heap> = RefCell::new(Heap::new()));

pub fn manage<T: 'static + Trace>(data: T) -> Root<T> {
    collect_if_needed();
    add_bytes::<T>();
    HEAP.with(|heap| heap.borrow_mut().manage(data))
}

pub fn unique<T: 'static + Trace>(data: T) -> UniqueRoot<T> {
    collect_if_needed();
    add_bytes::<T>();
    HEAP.with(|heap| heap.borrow_mut().unique(data))
}

pub fn root<T: 'static + Trace + ?Sized>(obj: Gc<T>) -> Root<T> {
    HEAP.with(|heap| heap.borrow_mut().root(obj))
}

//TODO Currently unused
// pub fn force_collect() {
//     STATS.with(|stats| {
//         let mut stats = stats.borrow_mut();
//         stats.bytes_allocated -= collect();
//     })
// }

fn collect() -> usize {
    HEAP.with(|heap| heap.borrow_mut().collect())
}

fn collect_if_needed() {
    STATS.with(|stats| {
        let mut stats = stats.borrow_mut();

        if stats.bytes_allocated > stats.threshold {
            stats.bytes_allocated -= collect();

            stats.threshold = (stats.bytes_allocated as f32 * 1.4) as usize;
        }
    })
}

fn add_bytes<T>() {
    STATS.with(|stats| {
        let mut stats = stats.borrow_mut();
        stats.bytes_allocated += std::mem::size_of::<T>();
    })
}
