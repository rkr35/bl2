mod array;
pub use array::Array;

mod array_property;
pub use array_property::ArrayProperty;

mod byte_property;
pub use byte_property::ByteProperty;

mod bool_property;
pub use bool_property::BoolProperty;

mod class;
pub use class::Class;

mod class_property;
pub use class_property::ClassProperty;

mod r#const;
pub use r#const::Const;

mod delegate_property;
pub use delegate_property::DelegateProperty;

mod r#enum;
pub use r#enum::Enum;

mod field;
pub use field::Field;

mod fstring;
pub use fstring::FString;

mod function;
pub use function::Function;

mod interface_property;
pub use interface_property::InterfaceProperty;

mod map_property;
pub use map_property::MapProperty;

mod name;
pub use name::{Entry, Name};

mod object;
pub use object::Object;

mod object_property;
pub use object_property::ObjectProperty;

mod property;
pub use property::Property;

mod script_struct;
pub use script_struct::ScriptStruct;

mod r#struct;
pub use r#struct::Struct;

mod struct_property;
pub use struct_property::StructProperty;


/// #Safety
/// Only as safe as you make it. If `object` is not a valid `&T` then you have
/// undefined behavior because this method will unconditionally cast `object` from 
/// `&Object` to `&To`.
pub unsafe fn cast<'a, To>(object: &'a Object<'a>) -> &'a To {
    &*(object as *const Object as *const To)
}