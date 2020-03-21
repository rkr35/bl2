use bl2_core::{
    game::Const,
    globals::Globals,
};

use std::ffi::OsString;

pub struct Constant<'a> {
    pub name: &'a str,
    pub value: OsString,
}

impl<'a> Constant<'a> {
    pub fn from<'n>(constant: &Const, globals: &'n Globals) -> Option<Constant<'n>> {
        let name = constant.name(globals.names)?;

        if name.contains("Default__") {
            return None;
        }

        Some(Constant {
            name,
            value: constant.value.to_string(),
        })
    }
}
