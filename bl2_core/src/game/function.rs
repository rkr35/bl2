use super::{Name, Struct};
use core::ffi::c_void as Void;
use core::ops::{Deref, DerefMut};

pub struct Function<'a> {
    r#struct: Struct<'a>,
    flags: u32,
    native: u16,
    rep_offset: u16,
    friendly_name: Name,
    pad0: [u8; 0x10],
    func: Option<&'a Void>,
}

impl<'a> Deref for Function<'a> {
    type Target = Struct<'a>;

    fn deref(&self) -> &Self::Target {
        &self.r#struct
    }
}

impl<'a> DerefMut for Function<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.r#struct
    }
}