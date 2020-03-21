use bl2_core::game::Object;

mod constant;
pub use constant::Constant;

mod enumeration;
pub use enumeration::Enumeration;

use std::collections::HashMap;

#[derive(Default)]
pub struct Package<'a> {
    pub subpackages: HashMap<&'a Object<'a>, SubPackage<'a>>,
}

#[derive(Default)]
pub struct SubPackage<'a> {
    pub enums: Vec<Enumeration<'a>>,
    pub consts: Vec<Constant<'a>>,
}
