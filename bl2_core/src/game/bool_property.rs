use super::Property;
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct BoolProperty<'a> {
    property: Property<'a>,
    pub bitmask: u32,
}

impl<'a> Deref for BoolProperty<'a> {
    type Target = Property<'a>;

    fn deref(&self) -> &Self::Target {
        &self.property
    }
}

impl<'a> DerefMut for BoolProperty<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.property
    }
}