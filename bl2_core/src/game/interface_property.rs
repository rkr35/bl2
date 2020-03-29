use super::{Class, Property};
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct InterfaceProperty<'a> {
    property: Property<'a>,
    pub class: Option<&'a Class<'a>>,
}

impl<'a> Deref for InterfaceProperty<'a> {
    type Target = Property<'a>;

    fn deref(&self) -> &Self::Target {
        &self.property
    }
}

impl<'a> DerefMut for InterfaceProperty<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.property
    }
}