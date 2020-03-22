use bl2_core::{
    game::{cast, Class, Const, Enum, Object},
    globals::{self, Globals},
};
use bl2_macros::main;
use log::{error, info};
use std::collections::HashMap;

use thiserror::Error;

mod config;
use config::Config;

mod static_classes;
use static_classes::StaticClasses;

mod staging;

#[derive(Error, Debug)]
enum Error {
    #[error("Globals error: {source}")]
    Globals {
        #[from]
        source: globals::Error,
    },

    #[error("Static classes error: {source}")]
    UnableToFindStaticClasses {
        #[from]
        source: static_classes::Error,
    },

    #[error("Unable to find outer class for {0:#x}")]
    UnableToFindOuter(usize),
}

fn process_packages(_config: &Config, globals: &Globals) -> Result<(), Error> {
    info!("Looking for static_classes.");
    let static_classes = StaticClasses::new(globals)?;
    info!("Found static_classes.");
    let mut packages = HashMap::<&Object, staging::Package>::new();

    // try_cast<Enum>(object, static_classes.enumeration)
    for object in globals.non_null_objects_iter() {
        if let Some(package) = object.package() {
            macro_rules! pkg {
                () => {
                    packages.entry(package).or_default()
                };
            }

            macro_rules! outer {
                () => {
                    object.outer.ok_or(Error::UnableToFindOuter(object as *const _ as usize))?
                };
            }

            macro_rules! pkg_outer {
                () => {
                    pkg!().subpackages.entry(outer!()).or_default()
                }
            }

            if object.is(static_classes.enumeration) {
                let e = staging::Enum::from(unsafe { cast::<Enum>(object) }, globals);
                if let Some(e) = e {
                    pkg_outer!().enums.push(e);
                }
            } else if object.is(static_classes.constant) {
                let c = staging::Const::from(unsafe { cast::<Const>(object) }, globals);
                if let Some(c) = c {
                    pkg_outer!().consts.push(c);               
                }
            } else if object.is(static_classes.class) {
                let cl = staging::Class::from(unsafe { cast::<Class>(object) }, globals);
                if let Some(cl) = cl {

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
