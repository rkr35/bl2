use super::Property;
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct ArrayProperty<'a> {
    property: Property<'a>,
    pub inner: Option<&'a Property<'a>>,
}

impl<'a> Deref for ArrayProperty<'a> {
    type Target = Property<'a>;

    fn deref(&self) -> &Self::Target {
        &self.property
    }
}

impl<'a> DerefMut for ArrayProperty<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.property
    }
}