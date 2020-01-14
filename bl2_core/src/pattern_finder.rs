use core::fmt::{self, Display};
use thiserror::Error;
use winapi::{
    um::{
        libloaderapi::GetModuleHandleW,
        errhandlingapi::{GetLastError, SetLastError},
    },
};

#[derive(Debug)]
pub struct WinApiErrorCode {
    error: u32,
}

impl From<u32> for WinApiErrorCode {
    fn from(error: u32) -> Self {
        Self { error }
    }
}

impl Display for WinApiErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.error.fmt(f)
    }
}

macro_rules! winapi {
    ($call:expr) => {{
        const SUCCESS: u32 = 0;

        // Set this thread's last-error value to a known success state so that
        // we can later query the error-code after a winapi call to determine
        // whether failure occurred.
        unsafe { SetLastError(SUCCESS); }

        let ret = || $call;
        let ret = ret();

        let error_code = unsafe { GetLastError() };

        if error_code == SUCCESS {
            Ok(ret)
        } else {
            Err(WinApiErrorCode::from(error_code))
        }
    }}
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Module not found. Are you sure it's running and that the name \
        is correctly spelled? GetLastError() == {0}")]
    ModuleNotFound(WinApiErrorCode),

    #[error("Failed to get module information. GetLastError() == {0}")]
    FailedToGetModuleInformation(WinApiErrorCode),
}

struct Module {
    start: usize,
    end: usize,
}

pub type Pattern = ();

pub struct PatternFinder {
    module: Module,
}

impl PatternFinder {
    pub fn new(module_name: &[u16]) -> Result<Self, Error> {

        let module = winapi!(unsafe { GetModuleHandleW(module_name.as_ptr()) });
        let module = module.map_err(Error::ModuleNotFound)?;

        todo!();
    }

    pub fn find<T>(&self, pattern: Pattern) -> Option<&'static mut T> {
        todo!();
    }
}
