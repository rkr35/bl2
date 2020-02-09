#[repr(C)]
pub struct Field<'a> {
    next: Option<&'a Field<'a>>,
}