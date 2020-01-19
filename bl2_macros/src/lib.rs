#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

extern crate proc_macro;

use proc_macro::TokenStream as OldTokenStream;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn main(input: OldTokenStream) -> OldTokenStream {
    let input = parse_macro_input!(input as TokenStream);

    let generated = quote! {
        use winapi::{
            shared::minwindef::{BOOL, DWORD, FALSE, HINSTANCE, LPVOID, TRUE},
        };

        extern "system" fn on_attach(dll: LPVOID) -> DWORD {
            use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};
            use std::panic;
            use winapi::{
                um::{
                    consoleapi::AllocConsole,
                    libloaderapi::FreeLibraryAndExitThread,
                    wincon::FreeConsole,
                }
            };
            
            let result = panic::catch_unwind(|| {
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
            });

            if let Err(panic) = result {
                log::error!("on_attach() caught a panic. The state of the hook \
                    is unknown. The hook will now detach.");
                bl2_core::idle();
            }

            unsafe {
                FreeConsole();
                FreeLibraryAndExitThread(dll.cast(), 0);
            }
        
            0
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        extern "system" fn DllMain(dll: HINSTANCE, reason: DWORD, _: LPVOID)
            -> BOOL {
            use bl2_core::{wide_format, winapi};
            use bl2_core::winapi_helpers::WinApiErrorCode;
            use core::ptr::null_mut;
            use winapi::{
                um::{
                    libloaderapi::DisableThreadLibraryCalls, 
                    processthreadsapi::CreateThread,
                    winnt::DLL_PROCESS_ATTACH,
                    winuser::{MB_OK, MessageBoxW},
                },
            };

            if reason != DLL_PROCESS_ATTACH {
                return TRUE;
            }

            let api_result = unsafe { winapi!(DisableThreadLibraryCalls, dll) };

            if let Err(error_code) = api_result {
                use wchar::wch_c as w;
                
                let hwnd = null_mut();
                
                let text = wide_format!("DisableThreadLibraryCalls() failed. \
                    GetLastError = {:#x}", error_code);
                    
                let caption = w!("Error");

                unsafe { MessageBoxW(hwnd, text.as_ptr(), caption.as_ptr(),
                    MB_OK) };

                FALSE
            } else {
                unsafe { CreateThread(null_mut(), 0, Some(on_attach),
                    dll.cast(), 0, null_mut()) };
                    
                TRUE
            }
        }
    };

    generated.into()
}