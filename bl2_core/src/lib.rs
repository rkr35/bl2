#![warn(clippy::pedantic)]

use log::info;

pub fn idle() {
    use std::io::{self, Read};
    info!("Idling. Press enter to continue.");
    let mut sentinel = [0; 2];
    let _ = io::stdin().read_exact(&mut sentinel);
}