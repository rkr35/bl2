#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use bl2_macros::main;

main! {
    bl2_core::idle();
}