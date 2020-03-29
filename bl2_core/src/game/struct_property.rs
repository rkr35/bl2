use super::{Property, Struct};
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct StructProperty<'a> {
    property: Property<'a>,
    pub r#struct: Option<&'a Struct<'a>>,
}

impl<'a> Deref for StructProperty<'a> {
    type Target = Property<'a>;

    fn deref(&self) -> &Self::Target {
        &self.property
    }
}

impl<'a> DerefMut for StructProperty<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.property
    }
}