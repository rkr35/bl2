use super::{Name, Struct};
use core::ffi::c_void as Void;
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct Function<'a> {
    r#struct: Struct<'a>,
    flags: u32,
    native: u16,
    rep_offset: u16,
    friendly_name: Name,
    operator_precedence: u8,
    num_params: u8,
    params_size: u16,
    ret_value_offset: u16,
    pad0: [u8; 6],
    func: Option<&'a Void>,
    pad1: [u8; 4],
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