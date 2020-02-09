use log::info;

pub mod array;
pub mod globals;
pub mod name;
pub mod object;
pub mod pattern;
pub mod winapi_helpers;

pub fn idle() {
    use std::io::{self, Read};
    info!("Idling. Press enter to continue.");
    let mut sentinel = [0; 2];
    let _ = io::stdin().read_exact(&mut sentinel);
}