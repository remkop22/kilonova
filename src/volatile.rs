use core::ops::{Deref, DerefMut};
use core::ptr;

#[repr(transparent)]
pub struct Volatile<R> {
    target: R,
}

impl<R> Volatile<R> {
    pub fn new(target: R) -> Self {
        Self { target }
    }
}

impl<R> Volatile<R> {

	pub fn read(&self) -> R {
		unsafe { ptr::read_volatile(&self.target ) }
	}

	pub fn write(&mut self, source: R){
		unsafe { ptr::write_volatile(&mut self.target, source) }
	}

    pub fn read_deref<T>(&self) -> T
    where
        R: Deref<Target = T>,
    {
        unsafe { ptr::read_volatile(&*self.target) }
    }

    pub fn write_deref<T>(&mut self, source: T)
    where
        R: DerefMut<Target = T>,
    {
        unsafe { ptr::write_volatile(&mut *self.target, source) }
    }
}
