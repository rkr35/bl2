use super::{Enum, Property};
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct ByteProperty<'a> {
    property: Property<'a>,
    pub r#enum: Option<&'a Enum<'a>>,
}

impl<'a> Deref for ByteProperty<'a> {
    type Target = Property<'a>;

    fn deref(&self) -> &Self::Target {
        &self.property
    }
}

impl<'a> DerefMut for ByteProperty<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.property
    }
}