#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use bl2_core::pattern::{self, Finder};
use bl2_macros::main;
use log::{error, info};
use thiserror::Error;
use wchar::wch_c as L;

#[derive(Error, Debug)]
enum Error {
    #[error("Unable to find address of global names.")]
    NamesNotFound,

    #[error("Unable to find address of global objects.")]
    ObjectsNotFound,

    #[error("Pattern finder error: {source}")]
    Finder {
        #[from]
        source: pattern::Error
    }
}

#[derive(Debug)]
struct GlobalNamesAndObjects {
    names: usize,
    objects: usize,
}

impl GlobalNamesAndObjects {
    fn new() -> Result<Self, Error> {
        let finder = Finder::new(L!("Borderlands2.exe"))?;
        let names = finder.find_names()?.ok_or(Error::NamesNotFound)?;
        let objects = finder.find_objects()?.ok_or(Error::ObjectsNotFound)?;
        Ok(Self { names, objects })
    }
}

main! {
    match GlobalNamesAndObjects::new() {
        Ok(globals) => info!("{:#x?}", globals),
        Err(e) => error!("{}", e),
    };
    bl2_core::idle();
}