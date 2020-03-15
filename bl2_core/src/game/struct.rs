use core::ops::{Deref, DerefMut};
use super::Field;

#[repr(C)]
pub struct Struct<'a> {
    field: Field<'a>,
    pad0: u64,
    pub super_field: Option<&'a Struct<'a>>,
    pub children: Option<&'a Field<'a>>,
    pub property_size: u16,
    pad1: [u8; 0x3a],
}

impl<'a> Deref for Struct<'a> {
    type Target = Field<'a>;

    fn deref(&self) -> &Self::Target {
        &self.field
    }
}

impl<'a> DerefMut for Struct<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.field
    }
}