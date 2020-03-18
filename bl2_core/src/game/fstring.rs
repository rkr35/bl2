use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use super::Array;

pub type FString<'a> = Array<'a, u16>; // &[u16] -> OsString -> Cow<str>

impl<'a> FString<'a> {
    pub fn to_string(&self) -> OsString {
         OsString::from_wide(&self)
    }
}