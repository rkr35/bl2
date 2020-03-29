use super::Struct;
use core::ops::{Deref, DerefMut};

pub struct ScriptStruct<'a> {
    r#struct: Struct<'a>,
    pad0: [u8; 0x1c],
}

impl<'a> Deref for ScriptStruct<'a> {
    type Target = Struct<'a>;

    fn deref(&self) -> &Self::Target {
        &self.r#struct
    }
}

impl<'a> DerefMut for ScriptStruct<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.r#struct
    }
}