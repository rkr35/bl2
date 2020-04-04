use super::Object;
use core::ffi::c_void as Void;

#[repr(C)]
pub struct ScriptInterface<'a> {
    object: Option<&'a Object<'a>>,
    interface: Option<&'a Void>,
}