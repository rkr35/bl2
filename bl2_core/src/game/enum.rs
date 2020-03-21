use core::ops::{Deref, DerefMut};
use crate::globals::Names;
use super::{Array, Field, Name};

#[repr(C)]
pub struct Enum<'a> {
    field: Field<'a>,
    variants: Array<'a, Name>,
}

impl<'a> Enum<'a> {
    pub fn variants<'n>(&self, global_names: &'n Names)
        -> Option<Vec<&'n str>> {
        self
            .variants
            .iter()
            .map(|n| n
                .entry(global_names)
                .and_then(|entry| entry.to_str().ok())
            )
            .collect()
    }
}

impl<'a> Deref for Enum<'a> {
    type Target = Field<'a>;

    fn deref(&self) -> &Self::Target {
        &self.field
    }
}

impl<'a> DerefMut for Enum<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.field
    }
}
