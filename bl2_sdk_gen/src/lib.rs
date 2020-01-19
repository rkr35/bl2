#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use bl2_core::pattern::Finder;
use bl2_macros::main;
use log::{error, info};
use wchar::wch_c as L;

main! {
    match Finder::new(L!("Borderlands2.exe")) {
        Ok(finder) => {
            let global_names = finder.find_names();
            info!("{:#x?}", global_names);
        }

        Err(e) => {
            error!("{}", e);
        }
    }

    bl2_core::idle();
}