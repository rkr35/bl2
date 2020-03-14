use crate::game::{Array, Entry, Object};
use crate::pattern::{self, Byte, Finder};
use log::info;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
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
    },
    
    #[error("Io error: {source}")]
    Io {
        #[from]
        source: io::Error,
    }
}

pub type Names<'n> = Array<'n, Option<&'n Entry>>;
type Objects<'o> = Array<'o, Option<&'o mut Object<'o>>>;

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

pub struct Globals {
    pub names: &'static Names<'static>,
    pub objects: &'static mut Objects<'static>,
}

impl Globals {
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

    fn dump_names(&self, output: &Path) -> Result<(), Error> {
        info!("Creating file {}", 
            output.file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("BAD FILE NAME"));
        
        let mut file = File::create(output).map(BufWriter::new)?;
        
        writeln!(&mut file, "Global names address: {:#x}",
            self.names as *const _ as usize)?;
        
        info!("Dumping names.");
        for (i, name) in self.names.iter().enumerate() {
            if let Some(name) = name {
                let name = name
                    .to_str()
                    .unwrap_or("not a valid utf-8 string");
                writeln!(&mut file, "[{}] {}", i, name)?;
            } else {
                writeln!(&mut file, "[{}] !null!", i)?;
            }
        }
        info!("Done dumping names.");

        Ok(())
    }

    fn dump_objects(&self, output: &Path) -> Result<(), Error> {
        info!("Creating file {}", 
            output.file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("BAD FILE NAME"));
        
        let mut file = File::create(output).map(BufWriter::new)?;
        
        writeln!(&mut file, "Global objects address: {:#x}",
            self.objects as *const _ as usize)?;
        
        info!("Dumping objects.");
        for (i, object) in self.objects.iter().enumerate() {
            if let Some(object) = object {
                let address = object as *const _ as usize;
                if let Some(name) = object.entry(self.names) {
                    let name = name
                        .to_str()
                        .unwrap_or("not a valid utf-8 string");
                    writeln!(&mut file, "[{}] {} {:#x}", i, name, address)?;
                } else {
                    writeln!(&mut file, "[{}] name not found for object at \
                        {:#x}", i, address)?;
                }
            } else {
                writeln!(&mut file, "[{}] !null!", i)?;
            }
        }
        info!("Done dumping objects.");

        Ok(())
    }

    pub fn dump(&self, output_directory: &Path) -> Result<(), Error> {
        create_directory(output_directory)?;
        self.dump_names(&output_directory.join("names_dump.txt"))?;
        self.dump_objects(&output_directory.join("objects_dump.txt"))?;
        Ok(())
    }
}

fn create_directory(directory: &Path) -> Result<(), Error> {
    info!("Creating directory {}", directory.to_string_lossy());
    fs::create_dir_all(directory)?;
    Ok(())
}