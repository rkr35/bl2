use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;
use core::slice;

#[repr(C)]
pub struct TArray<'a, T> {
    data: Option<&'a mut T>,
    count: u32,
    max: u32,
}

impl<'a, T> Deref for TArray<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        if let Some(data) = &self.data {
            unsafe {
                slice::from_raw_parts(*data, self.count as usize) 
            }
        } else {
            unsafe {
                slice::from_raw_parts(NonNull::dangling().as_ptr(), 0)
            }
        }
    }
}

impl<'a, T> DerefMut for TArray<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let Some(data) = &mut self.data {
            unsafe {
                slice::from_raw_parts_mut(*data, self.count as usize) 
            }
        } else {
            unsafe {
                slice::from_raw_parts_mut(NonNull::dangling().as_ptr(), 0)
            }
        }
    }
}