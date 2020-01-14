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
    ($function:expr, $($arg:tt)*) => {{
        const SUCCESS: u32 = 0;

        unsafe fn you_must_wrap_the_macro_in_unsafe() {}
        you_must_wrap_the_macro_in_unsafe();

        // Set this thread's last-error value to a known success state so that
        // we can later query the error-code after a winapi call to determine
        // whether failure occurred.
        SetLastError(SUCCESS);

        let ret = ($function) ($($arg)*);

        let error_code = GetLastError();

        if error_code == SUCCESS {
            Ok(ret)
        } else {
            Err(WinApiErrorCode::from(error_code))
        }
    }};
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to get module handle. Are you sure it's running and that \
        the name is correctly spelled? GetLastError() == {0}")]
    GetModuleHandleFailed(WinApiErrorCode),

    #[error("Failed to get module information. GetLastError() == {0}")]
    GetModuleInformationFailed(WinApiErrorCode),
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
        let module = unsafe { winapi!(GetModuleHandleW, module_name.as_ptr()) };
        let module = module.map_err(Error::GetModuleHandleFailed)?;
        todo!();
    }

    pub fn find<T>(&self, pattern: Pattern) -> Option<&'static mut T> {
        todo!();
    }
}
