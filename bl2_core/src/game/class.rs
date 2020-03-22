use super::{Object, Struct};
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct Class<'a> {
    r#struct: Struct<'a>,
    pad0: [u8; 0x60],
    class_default_object: Option<&'a Object<'a>>,
    pad1: [u8; 0x5C],
}

impl<'a> Deref for Class<'a> {
    type Target = Struct<'a>;

    fn deref(&self) -> &Self::Target {
        &self.r#struct
    }
}

impl<'a> DerefMut for Class<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.r#struct
    }
}
