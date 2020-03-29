use super::{Function, Property};
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct DelegateProperty<'a> {
    property: Property<'a>,
    pub signature_function: Option<&'a Function<'a>>,
    pub unknown: Option<&'a Function<'a>>,
}

impl<'a> Deref for DelegateProperty<'a> {
    type Target = Property<'a>;

    fn deref(&self) -> &Self::Target {
        &self.property
    }
}

impl<'a> DerefMut for DelegateProperty<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.property
    }
}