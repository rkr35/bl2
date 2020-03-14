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
    }
}

struct _Package {

}

fn process_packages(_config: &Config, globals: &Globals) -> Result<(), Error> {
    // let mut packages = vec![false, true];
    // let mut processed_objects = HashMap::<usize, bool>::new();
    let package_objects: HashSet<_> = globals
        .objects
        .iter()
        .filter_map(|o| o.as_ref().and_then(|o| o.get_package()))
        .collect();

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