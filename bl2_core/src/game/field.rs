use super::Object;
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct Field<'a> {
    object: Object<'a>,
    next: Option<&'a Field<'a>>,
}

impl<'a> Deref for Field<'a> {
    type Target = Object<'a>;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<'a> DerefMut for Field<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}
