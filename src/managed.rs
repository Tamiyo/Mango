use core::cell::Cell;
use core::ops::Deref;
use core::ops::DerefMut;
use core::ptr;
use core::ptr::NonNull;
use std::fmt;

#[derive(Debug, Default)]
pub struct Header {
    marked: Cell<bool>,
}

#[derive(Debug)]
pub struct Allocation<T: 'static + ?Sized> {
    header: Header,
    data: T,
}

impl<T: 'static> Allocation<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            header: Header {
                marked: Cell::new(false),
            },
        }
    }
}

pub struct Managed<T: 'static + ?Sized> {
    ptr: NonNull<Allocation<T>>,
}

impl<T: 'static + fmt::Debug> fmt::Debug for Managed<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inner: &T = &*self;

        f.debug_struct("Managed").field("ptr", inner).finish()
    }
}

impl<T: 'static + ?Sized> Managed<T> {
    /// Note this method is hilariously unsafe
    pub unsafe fn deref_static(&self) -> &'static T {
        &(*self.ptr.as_ptr()).data
    }

    pub fn obj(&self) -> &Allocation<T> {
        unsafe { self.ptr.as_ref() }
    }
    pub fn obj_mut(&mut self) -> &mut Allocation<T> {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: 'static + ?Sized> From<NonNull<Allocation<T>>> for Managed<T> {
    fn from(fun: NonNull<Allocation<T>>) -> Self {
        Self { ptr: fun }
    }
}

impl<T: 'static + ?Sized> Copy for Managed<T> {}
impl<T: 'static + ?Sized> Clone for Managed<T> {
    fn clone(&self) -> Managed<T> {
        *self
    }
}

impl<T: 'static> Deref for Managed<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.obj().data
    }
}

impl<T: 'static> DerefMut for Managed<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.obj_mut().data
    }
}

impl<T: 'static> PartialEq for Managed<T> {
    fn eq(&self, other: &Managed<T>) -> bool {
        let left_inner: &T = &*self;
        let right_inner: &T = &*other;

        ptr::eq(left_inner, right_inner)
    }
}

pub fn make_managed<T: 'static>(data: T) -> Managed<T> {
    let mut alloc = Box::new(Allocation::new(data));
    let ptr = unsafe { NonNull::new_unchecked(&mut *alloc) };
    Managed::from(ptr)
}
