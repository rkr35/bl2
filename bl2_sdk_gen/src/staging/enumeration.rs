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

#[derive(Debug)]
pub struct Enumeration<'a> {
    pub name: &'a str,
    pub full_name: String,
    pub variants: Vec<&'a str>,
}

impl<'a> Enumeration<'a> {
    pub fn from<'n>(enumeration: &Enum, globals: &'n Globals)
        -> Option<Enumeration<'n>> {

        let name = enumeration.name(globals.names)?;
        
        if name.contains("Default__") {
            return None;
        }
        
        Some(Enumeration {
            name,
            full_name: enumeration.full_name(globals.names)?,
            variants: enumeration.variants(globals.names)?,
        })
    }
}
