#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use bl2_core::pattern::GlobalNamesAndObjects;
use bl2_macros::main;
use log::{error, info};

main! {
    match GlobalNamesAndObjects::new() {
        Ok(globals) => info!("{:#x?}", globals),
        Err(e) => error!("{}", e),
    };
    bl2_core::idle();
}