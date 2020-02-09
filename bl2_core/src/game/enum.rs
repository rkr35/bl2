use super::{Array, Name};

#[repr(C)]
pub struct Enum<'a> {
    names: Array<'a, Name>,
}