use core::convert::TryFrom;
use core::mem::{MaybeUninit, size_of};
use crate::{winapi, winapi_helpers::{WinApiErrorCode}};
use log::info;
use thiserror::Error;
use ::winapi::{
    shared::minwindef::HMODULE as Module,
    um::{
        libloaderapi::GetModuleHandleW,
        processthreadsapi::GetCurrentProcess,
        psapi::{
            GetModuleInformation,
            MODULEINFO as ModuleInfo,
        },
    },
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to get module handle. Are you sure it's running and that \
        the name is correctly spelled? GetLastError() == {0}")]
    GetModuleHandleFailed(WinApiErrorCode),

    #[error("Failed to get module information. GetLastError() == {0}")]
    GetModuleInformationFailed(WinApiErrorCode),

    #[error("Attempted lossy integer cast when trying to convert {action} to \
        {dest_type}. The source value is {source_value}, but the destination \
        type \"{dest_type}\" only has a range of [{min}, {max}].")]
    LossyIntCast {
        action: &'static str,
        source_value: String,
        dest_type: &'static str,
        min: String,
        max: String,
    }
}

macro_rules! try_int_cast {
    ($from:expr, $to:ty, $action:literal) => {{
        <$to>::try_from($from)
            .map_err(|_| Error::LossyIntCast {
                action: $action,
                source_value: ($from).to_string(),
                dest_type: stringify!($to),
                min: <$to>::min_value().to_string(),
                max: <$to>::max_value().to_string(),
            })
    }}
}

pub enum Byte {
    Wildcard,
    Literal(u8),
}

pub struct PatternFinder {
    start: usize,
    end: usize,
}

fn get_module(name: &[u16]) -> Result<Module, Error> {
    let module = unsafe { winapi!(GetModuleHandleW, name.as_ptr()) };
    module.map_err(Error::GetModuleHandleFailed)
}

fn get_module_info(name: &[u16]) -> Result<ModuleInfo, Error> {
    let module = get_module(name)?;
    let process = unsafe { GetCurrentProcess() };
    
    let mut module_info = MaybeUninit::<ModuleInfo>::uninit();
    let module_info_size = try_int_cast!(size_of::<ModuleInfo>(), u32, 
        "size of ModuleInfo")?;

    unsafe {
        let module_info = module_info.as_mut_ptr();
        winapi!(GetModuleInformation, process, module, module_info,
            module_info_size).map_err(Error::GetModuleInformationFailed)?;
    }

    Ok(unsafe { module_info.assume_init() })
}

impl PatternFinder {
    pub fn new(module_name: &[u16]) -> Result<Self, Error> {
        let module_info = get_module_info(module_name)?;

        let start = module_info.lpBaseOfDll as usize;
        
        let size = try_int_cast!(module_info.SizeOfImage, usize,
            "size of module")?;
        
        let end = start + size;
        
        info!("[{:#x}, {:#x}) is {:#x} bytes.", start, end, size);
        
        Ok(Self {
            start,
            end,
        })
    }

    pub fn find<T>(&self, _pattern: &[Byte]) -> Option<&'static mut T> {
        todo!();
    }
}
