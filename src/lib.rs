#![warn(clippy::pedantic)]

use bl2_macros::main;
use winapi::{
    um::{
        consoleapi::AllocConsole,
        wincon::FreeConsole,
    },
};

fn idle() {
    use std::io::{self, Read};
    // info!("Idling. Press enter to continue.");

    let mut sentinel = [0; 2];

    match io::stdin().read_exact(&mut sentinel) {
        _ => (),
    }
}

main! {
    unsafe {
        AllocConsole();
    }
    
    println!("Hey, got this .DLL injected!");
    idle();

    unsafe {
        FreeConsole();
    }
}