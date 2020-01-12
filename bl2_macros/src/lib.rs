#![warn(clippy::pedantic)]
extern crate proc_macro;

use proc_macro::TokenStream as OldTokenStream;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn main(input: OldTokenStream) -> OldTokenStream {
    let input = parse_macro_input!(input as TokenStream);

    let on_attach = quote! {
        extern "system" fn on_attach(dll: LPVOID) -> DWORD {
            use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};
            use winapi::{
                um::{
                    consoleapi::AllocConsole,
                    libloaderapi::FreeLibraryAndExitThread,
                    wincon::FreeConsole,
                }
            };

            unsafe {
                AllocConsole();
            }
            
            println!("Allocated console.");

            let filter = LevelFilter::Info;
            let config = Config::default();
            let mode = TerminalMode::Mixed;
            if let Err(e) = TermLogger::init(filter, config, mode) {
                eprintln!("Failed to initialize TermLogger: {}", e);
                bl2_core::idle();
            } else {
                log::info!("Initialized logger.");
                #input
            }

            unsafe {
                FreeConsole();
                FreeLibraryAndExitThread(dll.cast(), 0);
            }
        
            0
        }
    };

    let dll_main = quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        extern "system" fn DllMain(dll: HINSTANCE, reason: DWORD, _: LPVOID)
            -> BOOL {
            use core::ptr::null_mut;
            use winapi::{
                um::{
                    libloaderapi::DisableThreadLibraryCalls, 
                    processthreadsapi::CreateThread,
                    winnt::DLL_PROCESS_ATTACH,
                },
            };

            if reason == DLL_PROCESS_ATTACH {
                unsafe {
                    DisableThreadLibraryCalls(dll);
                    CreateThread(null_mut(), 0, Some(on_attach), dll.cast(), 0,
                                 null_mut());
                }
            }

            TRUE
        }
    };

    let generated = quote! {
        use winapi::{
            shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
        };

        #on_attach
        #dll_main
    };

    generated.into()
}