use bl2_core::game::Object;

mod r#const;
pub use r#const::Const;

mod r#enum;
pub use r#enum::Enum;

mod class;
pub use class::Class;

use std::collections::HashMap;

#[derive(Default)]
pub struct Package<'a> {
    pub subpackages: HashMap<&'a Object<'a>, SubPackage<'a>>,
}

#[derive(Default)]
pub struct SubPackage<'a> {
    pub enums: Vec<Enum<'a>>,
    pub consts: Vec<Const<'a>>,
}
