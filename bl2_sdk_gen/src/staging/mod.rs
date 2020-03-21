mod enumeration;
pub use enumeration::Enumeration;

mod constant;
pub use constant::Constant;

#[derive(Default)]
pub struct Package<'a> {
    pub enums: Vec<Enumeration<'a>>,
    pub consts: Vec<Constant<'a>>,
}
