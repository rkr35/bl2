use bl2_core::globals::{self, Globals};
use bl2_core::game::{Enum, Object};
use bl2_macros::main;
use std::collections::{HashMap, hash_map::Entry, HashSet};
use log::{error, info};
use std::path::Path;
use thiserror::Error;

struct Config<'a> {
    output_directory: &'a Path,
}

impl<'a> Config<'a> {
    // todo: Read from file
    // todo: Builder
    fn new() -> Config<'static> {
        const OUTPUT_DIR: &str = r"C:\Users\Royce\source\repos\bl2\src\sdk";

        Config {
            output_directory: Path::new(OUTPUT_DIR),
        }
    }
}

#[derive(Error, Debug)]
enum Error {
    #[error("Globals error: {source}")]
    Globals {
        #[from]
        source: globals::Error,
    },

    #[error("Unable to find static class \"{missing_class}\".")]
    UnableToFindStaticClasses {
        missing_class: String,
    }
}

struct StaticClasses<'a> {
    pub enumeration: &'a Object<'a>,
    pub constant: &'a Object<'a>,
    pub class: &'a Object<'a>,
    pub script_struct: &'a Object<'a>,
}

impl<'a> StaticClasses<'a> {
    pub fn new(globals: &Globals) -> Result<StaticClasses, Error> {
        let find = |class: &str| globals
            .non_null_objects_iter()
            .find(|o| o.full_name(globals.names) == class)
            .ok_or_else(|| Error::UnableToFindStaticClasses {
                missing_class: class.to_string(),
            });

        Ok(StaticClasses {
            enumeration: find("Class Core.Enum")?,
            constant: find("Class Core.Const")?,
            class: find("Class Core.Class")?,
            script_struct: find("Class Core.ScriptStruct")?,
        })
    }
}

unsafe fn cast<'a, To>(object: &'a Object<'a>) -> &'a To {
    &*(object as *const Object as *const To)
}

#[derive(Debug)]
struct Enumeration<'a> {
    name: &'a str,
    full_name: String,
    variants: Vec<&'a str>,
}

fn make_enum<'n>(enumeration: &Enum, globals: &'n Globals) -> Option<Enumeration<'n>> {
    let name = enumeration
        .name(globals.names)
        .unwrap_or("BAD ENUM NAME");
    
    if name.contains("Default__") {
        return None;
    }

    Some(Enumeration {
        name,
        full_name: enumeration.full_name(globals.names),
        variants: enumeration.variants(globals.names),
    })
}

#[derive(Default)]
struct Package<'a> {
    pub enumerations: Vec<Enumeration<'a>>,
}

impl<'a> Package<'a> {
}

fn process_packages(_config: &Config, globals: &Globals) -> Result<(), Error> {
    info!("Looking for static_classes.");
    let static_classes = StaticClasses::new(globals)?;
    info!("Found static_classes.");
    let mut processed_objects = HashMap::<&Object, bool>::new();
    let mut packages = HashMap::<&Object, Package>::new();

    for object in globals.non_null_objects_iter() {
        if let Some(package) = object.package() {
            macro_rules! pkg { () => { packages.entry(package).or_default() } }

            if object.is(static_classes.enumeration) {
                let e = make_enum(unsafe { cast::<Enum>(object) }, globals);
                if let Some(e) = e {
                    pkg!().enumerations.push(e);
                }
            }
        }
    }

    Ok(())
}

fn generate_sdk() -> Result<(), Error> {
    let config = Config::new();
    let globals = Globals::new()?;
    globals.dump(config.output_directory)?;
    process_packages(&config, &globals)?;
    Ok(())
}

main! {
    if let Err(e) = generate_sdk() {
        error!("{}", e);
    }

    bl2_core::idle();
}