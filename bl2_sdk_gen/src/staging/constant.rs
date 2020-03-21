use bl2_core::{
    game::{cast, Const, Enum, Object},
    globals::{self, Globals},
};
use bl2_macros::main;
use log::{error, info};
use std::collections::{HashMap, hash_map::Entry, HashSet};
use std::ffi::OsString;
use std::path::Path;
use thiserror::Error;

pub struct Constant<'a> {
    pub name: &'a str,
    pub value: OsString,
}

impl<'a> Constant<'a> {
    pub fn from<'n>(constant: &Const, globals: &'n Globals)
        -> Option<Constant<'n>> {

        let name = constant.name(globals.names)?;

        if name.contains("Default__") {
            return None;
        }
    
        Some(Constant {
            name,
            value: constant.value.to_string()
        })
    }
}