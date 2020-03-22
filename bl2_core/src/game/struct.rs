use super::Field;
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct Struct<'a, Child = Field<'a>> {
    field: Field<'a>,
    pad0: [u8; 8],
    pub super_field: Option<&'a Struct<'a>>,
    pub children: Option<&'a Child>,
    pub property_size: u16,
    pad1: [u8; 0x3a],
}

impl<'a, Child> Deref for Struct<'a, Child> {
    type Target = Field<'a>;

    fn deref(&self) -> &Self::Target {
        &self.field
    }
}

impl<'a, Child> DerefMut for Struct<'a, Child> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.field
    }
}
