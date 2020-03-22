use bl2_core::{
    game::Const as GameConst,
    globals::Globals,
};

use std::ffi::OsString;

pub struct Const<'a> {
    pub name: &'a str,
    pub value: OsString,
}

impl<'a> Const<'a> {
    pub fn from<'n>(constant: &GameConst, globals: &'n Globals) -> Option<Const<'n>> {
        let name = constant.name(globals.names)?;

        if name.contains("Default__") {
            return None;
        }

        Some(Const {
            name,
            value: constant.value.to_string(),
        })
    }
}
