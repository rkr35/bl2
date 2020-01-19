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
    },

    #[error("Overflow on addition when calculating end-of-module address.\
        start = {start}, size = {size}")]
    OverflowEndOfModule {
        start: usize,
        size: usize,
    },

    #[error("Overflow on subtraction when calculating end of search space.\
        module end = {end}, pattern length = {pattern_length}")]
    OverflowSearchSpaceEnd {
        end: usize,
        pattern_length: usize,
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

pub const NAMES_PATTERN: &[Byte] = &[
    Byte::Literal(0x8B), Byte::Literal(0x0D),
    Byte::Wildcard, Byte::Wildcard, Byte::Wildcard, Byte::Wildcard,
    Byte::Literal(0x83), Byte::Literal(0x3C), Byte::Literal(0x81),
];

pub struct Finder {
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

impl Finder {
    pub fn new(module_name: &[u16]) -> Result<Self, Error> {
        let module_info = get_module_info(module_name)?;

        let start = module_info.lpBaseOfDll as usize;
        
        let size = try_int_cast!(module_info.SizeOfImage, usize,
            "size of module")?;
        
        let end = start
            .checked_add(size)
            .ok_or(Error::OverflowEndOfModule { start, size })?;
        
        info!("[{:#x}, {:#x}) is {:#x} bytes.", start, end, size);
        
        Ok(Self {
            start,
            end,
        })
    }

    fn is_match(address: usize, pattern: &[Byte]) -> bool {
        (address..)
            .map(|address| address as *const u8)
            .zip(pattern.iter())
            .all(|(byte, pattern)| match pattern {
                Byte::Wildcard => true,
                Byte::Literal(expected) => *expected == unsafe { *byte },
            })
    }

    pub fn find(&self, pattern: &[Byte]) -> Result<Option<usize>, Error> {
        let end = self.end.checked_sub(pattern.len())
            .ok_or(Error::OverflowSearchSpaceEnd {
                end: self.end,
                pattern_length: pattern.len(),
            })?;
        let mut search_space = self.start..end;
        Ok(search_space.find(|address| Self::is_match(*address, pattern)))
    }

    pub fn find_names(&self) -> Result<Option<usize>, Error> {
        // 00E8BD1C - 8B 0D 04022502        - mov ecx,[02250204]
        Ok(self.find(NAMES_PATTERN)?.map(|address| {
            let address = (address + 2) as *const usize;
            unsafe { *address }
        }))
    }
}