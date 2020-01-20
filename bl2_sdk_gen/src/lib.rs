#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use bl2_core::pattern::{self, GlobalNamesAndObjects};
use bl2_macros::main;
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
    #[error("Pattern finder error: {source}")]
    Pattern {
        #[from]
        source: pattern::Error,
    }
}

fn generate_sdk() -> Result<(), Error> {
    let config = Config::new();
    let globals = GlobalNamesAndObjects::new()?;
    globals.dump(config.output_directory)?;

    todo!();
}

main! {
    if let Err(e) = generate_sdk() {
        error!("{}", e);
    }

    bl2_core::idle();
}