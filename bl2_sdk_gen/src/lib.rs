#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use bl2_core::PatternFinder;
use bl2_macros::main;
use log::error;
use wchar::wch_c as L;

main! {
    match PatternFinder::new(L!("Borderlands2.exe")) {
        Ok(finder) => {

        }

        Err(e) => {
            error!("{}", e);
        }
    }

    bl2_core::idle();
}