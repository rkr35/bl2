use core::str::Utf8Error;
use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
pub struct Entry {
    pad0: [u8; 0x10],
    text: c_char,
}

impl Entry {
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        let ptr = &self.text as *const c_char;

        // Safe because:
        // 1. The returned lifetime is guaranteed to be the lifetime of &self.
        // An Entry inlines its text, so an Entry's text lives exactly as
        // long as the Entry itself.

        // 2. Every Entry's text is a NULL terminated C string.

        // 3. An Entry's text is immutable, so the contents of the CStr at
        //  creation remains unchanged for its lifetime.
        let c_str = unsafe { CStr::from_ptr(ptr) };

        c_str.to_str()
    }
}