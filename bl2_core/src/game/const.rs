use super::Array;

pub type UString<'a> = Array<'a, u16>; // &[u16] -> OsString -> Cow<str>

#[repr(C)]
pub struct Const<'a> {
    value: UString<'a>,
}