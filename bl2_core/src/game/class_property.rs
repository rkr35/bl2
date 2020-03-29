use super::{Class, ObjectProperty};
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct ClassProperty<'a> {
    object_property: ObjectProperty<'a>,
    pub meta_class: Option<&'a Class<'a>>,
}

impl<'a> Deref for ClassProperty<'a> {
    type Target = ObjectProperty<'a>;

    fn deref(&self) -> &Self::Target {
        &self.object_property
    }
}

impl<'a> DerefMut for ClassProperty<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object_property
    }
}