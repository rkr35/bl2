#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use log::info;

pub mod pattern;
pub mod tarray;
pub mod winapi_helpers;

pub fn idle() {
    use std::io::{self, Read};
    info!("Idling. Press enter to continue.");
    let mut sentinel = [0; 2];
    let _ = io::stdin().read_exact(&mut sentinel);
}