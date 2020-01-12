#![warn(clippy::pedantic)]

use bl2_macros::main;

main! {
    bl2_core::idle();
}