use bl2_core::game::Object;
use crate::staging::{Class, Const, Enum};

use std::collections::HashMap;

#[derive(Default)]
pub struct SubPackage<'a> {
    pub consts: Vec<Const<'a>>,
    pub enums: Vec<Enum<'a>>,
    pub classes: HashMap<&'a Object<'a>, Class<'a>>,
}