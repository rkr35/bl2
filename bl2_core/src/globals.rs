use crate::{
    array::Array,
    object::Object,
    name,
    pattern::{self, Byte, Finder},
};
use log::info;
use std::path::Path;
use thiserror::Error;
use wchar::wch_c as w;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to find address of global names.")]
    NamesNotFound,

    #[error("Unable to find address of global objects.")]
    ObjectsNotFound,

    #[error("Pattern finder error: {source}")]
    Pattern {
        #[from]
        source: pattern::Error,
    }
}

type Names<'n> = Array<'n, Option<&'n name::Entry>>;
type Objects<'o> = Array<'o, Option<&'o mut Object>>;

const NAMES_PATTERN: &[Byte] = &[
    Byte::Literal(0x8B), Byte::Literal(0x0D),
    Byte::Wildcard, Byte::Wildcard, Byte::Wildcard, Byte::Wildcard,
    Byte::Literal(0x83), Byte::Literal(0x3C), Byte::Literal(0x81),
];

const OBJECTS_PATTERN: &[Byte] = &[
    Byte::Literal(0x8B), Byte::Literal(0x0D),
    Byte::Wildcard, Byte::Wildcard, Byte::Wildcard, Byte::Wildcard,
    Byte::Literal(0x8B), Byte::Literal(0x3C), Byte::Literal(0x81),
    Byte::Literal(0x8B), Byte::Literal(0xB5),
];

pub struct GlobalNamesAndObjects {
    names: &'static Names<'static>,
    objects: &'static mut Objects<'static>,
}

impl GlobalNamesAndObjects {
    pub fn new() -> Result<Self, Error> {
        fn get_mov_src_operand(mov_instruction_address: usize) -> usize {
            let src_operand_address = (mov_instruction_address + 2)
                as *const usize;
            unsafe { *src_operand_address }
        }

        let finder = Finder::new(w!("Borderlands2.exe"))?;

        let names = finder
            .find(NAMES_PATTERN)?
            .map(get_mov_src_operand)
            .and_then(|address| unsafe { (address as *const Names).as_ref() })
            .ok_or(Error::NamesNotFound)?;
    
        let objects = finder
            .find(OBJECTS_PATTERN)?
            .map(get_mov_src_operand)
            .and_then(|address| unsafe { (address as *mut Objects).as_mut() })
            .ok_or(Error::ObjectsNotFound)?;

        Ok(Self { names, objects })
    }

    pub fn dump(&self, output_directory: &Path) -> Result<(), Error> {
        info!("n {:#x}", self.names as *const _ as usize);
        info!("o {:#x}", self.objects as *const _ as usize);
        Ok(())
    }
}