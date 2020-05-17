use crate::vm::gc::Trace;
use std::cell::Cell;
use std::fmt;
use std::ops::Deref;
use std::ptr::NonNull;
#[derive(Debug)]
pub struct Header {
    pub marked: Cell<bool>,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            marked: Cell::new(false),
        }
    }
}
#[derive(Debug)]
pub struct Allocation<T: 'static + Trace + ?Sized> {
    pub header: Header,
    pub data: T,
}

impl<T: 'static + Trace + ?Sized> Trace for Allocation<T> {
    fn trace(&self) {
        if !self.header.marked.replace(true) {
            self.data.trace();
        }
    }
}

pub struct Managed<T: 'static + Trace + ?Sized> {
    pub ptr: NonNull<Allocation<T>>,
}

impl<T: 'static + Trace + ?Sized> Managed<T> {
    fn allocation(&self) -> &Allocation<T> {
        unsafe { &self.ptr.as_ref() }
    }
}

impl<T: 'static + Trace + ?Sized> Copy for Managed<T> {}

impl<T: 'static + Trace + ?Sized> Clone for Managed<T> {
    fn clone(&self) -> Managed<T> {
        *self
    }
}

impl<T: 'static + Trace + PartialEq + ?Sized> PartialEq<Managed<T>> for Managed<T> {
    fn eq(&self, other: &Managed<T>) -> bool {
        self.ptr == other.ptr
    }
}

impl<T: 'static + Trace + ?Sized> Deref for Managed<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.allocation().data
    }
}

impl<T: 'static + Trace + ?Sized> Trace for Managed<T> {
    fn trace(&self) {
        self.allocation().trace();
    }
}

impl<T: 'static + fmt::Debug + Trace + ?Sized> std::fmt::Debug for Managed<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner: &T = &*self;
        write!(f, "Managed({:?})", inner)
    }
}
