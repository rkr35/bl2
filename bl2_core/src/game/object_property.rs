use super::{Class, Property};
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct ObjectProperty<'a> {
    property: Property<'a>,
    pub class: Option<&'a Class<'a>>,
}

impl<'a> Deref for ObjectProperty<'a> {
    type Target = Property<'a>;

    fn deref(&self) -> &Self::Target {
        &self.property
    }
}

impl<'a> DerefMut for ObjectProperty<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.property
    }
}