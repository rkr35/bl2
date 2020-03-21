use super::{FString, Field};
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct Const<'a> {
    pub field: Field<'a>,
    pub value: FString<'a>,
}

impl<'a> Deref for Const<'a> {
    type Target = Field<'a>;

    fn deref(&self) -> &Self::Target {
        &self.field
    }
}

impl<'a> DerefMut for Const<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.field
    }
}
