use bl2_core::globals::{self, Globals};
use bl2_core::game::{Object};
use bl2_macros::main;
use std::collections::{HashMap, HashSet};
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

fn process_packages(_config: &Config, globals: &Globals) -> Result<(), Error> {
    info!("Looking for static_classes.");
    let static_classes = StaticClasses::new(globals)?;
    info!("Found static_classes.");
    for o in package_objects {
        if let Some(name) = o.name(globals.names) {
            info!("{}", name);
        } else {
            let address = o as *const _ as usize;
            error!("null package name for object {:#x}", address);
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