use core::fmt::{self, Display, LowerHex};

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
        Display::fmt(&self.error, f)
    }
}

impl LowerHex for WinApiErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        LowerHex::fmt(&self.error, f)
    }
}

#[macro_export]
macro_rules! winapi {
    ($function:expr, $($arg:tt)*) => {{
        use winapi::um::errhandlingapi::{GetLastError, SetLastError};

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

#[macro_export]
macro_rules! wide_format {
    ($format:literal, $($arg:tt)*) => {{
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;

        let mut widened: Vec<u16> = OsStr::new(&format!($format, $($arg)*))
            .encode_wide()
            .map(|byte| if byte == 0 {
                const REPLACEMENT_CHARACTER: u16 = 0xFFFD;
                REPLACEMENT_CHARACTER
            } else {
                byte
            })
            .collect();

        let needs_null_terminator = widened
            .last()
            .map_or(true, |last| *last != 0);
            
        if needs_null_terminator {
            widened.push(0);
        }

        widened
    }}
}