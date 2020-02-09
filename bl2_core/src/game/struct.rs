use super::Field;

#[repr(C)]
pub struct Struct<'a> {
    pad0: [u8; 0x8],
    super_field: Option<&'a Field<'a>>,
    children: Option<&'a Field<'a>>,
    property_size: u16,
    pad1: [u8; 0x3a],
}