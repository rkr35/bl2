use super::Property;
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct MapProperty<'a> {
    property: Property<'a>,
    pub key: Option<&'a Property<'a>>,
    pub value: Option<&'a Property<'a>>,
}

impl<'a> Deref for MapProperty<'a> {
    type Target = Property<'a>;

    fn deref(&self) -> &Self::Target {
        &self.property
    }
}

impl<'a> DerefMut for MapProperty<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.property
    }
}