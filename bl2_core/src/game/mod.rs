mod array;
pub use array::Array;

mod bool_property;
pub use bool_property::BoolProperty;

mod class;
pub use class::Class;

mod r#const;
pub use r#const::Const;

mod r#enum;
pub use r#enum::Enum;

mod field;
pub use field::Field;

mod fstring;
pub use fstring::FString;

mod name;
pub use name::{Entry, Name};

mod object;
pub use object::Object;

mod property;
pub use property::Property;

mod r#struct;
pub use r#struct::Struct;

pub unsafe fn cast<'a, To>(object: &'a Object<'a>) -> &'a To {
    &*(object as *const Object as *const To)
}