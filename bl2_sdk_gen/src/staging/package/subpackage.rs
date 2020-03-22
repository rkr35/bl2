use crate::staging::{Const, Enum};

#[derive(Default)]
pub struct SubPackage<'a> {
    pub consts: Vec<Const<'a>>,
    pub enums: Vec<Enum<'a>>,
}