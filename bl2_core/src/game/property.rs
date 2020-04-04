use super::Field;
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct Property<'a> {
    field: Field<'a>,
    pub array_dim: u32,
    pub element_size: u32,
    property_flags: u64,
    property_size: u16,
    pad0: [u8; 0xE],
    pub offset: u32,
    next_link: Option<&'a Property<'a>>,
    pad1: [u8; 0x18],
}

impl<'a> Deref for Property<'a> {
    type Target = Field<'a>;

    fn deref(&self) -> &Self::Target {
        &self.field
    }
}

impl<'a> DerefMut for Property<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.field
    }
}