#![warn(clippy::pedantic)]
extern crate proc_macro;

use proc_macro::TokenStream as OldTokenStream;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn main(input: OldTokenStream) -> OldTokenStream {
    let input = parse_macro_input!(input as TokenStream);
    
    let imports = quote! {
        use core::ptr::null_mut;
        use winapi::{
            shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
            um::{
                libloaderapi::{DisableThreadLibraryCalls, 
                               FreeLibraryAndExitThread},
                processthreadsapi::CreateThread,
                winnt::DLL_PROCESS_ATTACH,
            },
        };
    };

    let on_attach = quote! {
        extern "system" fn on_attach(dll: LPVOID) -> DWORD {
            #input

            unsafe {
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
        #imports
        #on_attach
        #dll_main
    };

    generated.into()
}