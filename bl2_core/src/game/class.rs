use super::{Object, Property, Struct};
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct Class<'a> {
    r#struct: Struct<'a, Property<'a>>,
    pad0: [u8; 0xcc],
    class_default_object: Option<&'a Object<'a>>,
    pad1: [u8; 0x74],
}

impl<'a> Deref for Class<'a> {
    type Target = Struct<'a, Property<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.r#struct
    }
}

impl<'a> DerefMut for Class<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.r#struct
    }
}


// struct, 0x44, 0x60