use bl2_core::game::Object;

mod constant;
pub use constant::Constant;

mod class;
pub use class::Class;

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
