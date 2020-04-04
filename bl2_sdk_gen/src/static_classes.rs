use bl2_core::game::Object;
use bl2_core::globals::Globals;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to find static class \"{missing_class}\".")]
    UnableToFindClass { missing_class: String },
}

pub struct StaticClasses<'a> {
    pub enumeration: &'a Object<'a>,
    pub constant: &'a Object<'a>,
    pub class: &'a Object<'a>,
    pub script_struct: &'a Object<'a>,
    pub function: &'a Object<'a>,
    pub bool_property: &'a Object<'a>,
    pub byte_property: &'a Object<'a>,
    pub int_property: &'a Object<'a>,
    pub float_property: &'a Object<'a>,
    pub object_property: &'a Object<'a>,
    pub component_property: &'a Object<'a>,
    pub class_property: &'a Object<'a>,
    pub interface_property: &'a Object<'a>,
    pub name_property: &'a Object<'a>,
    pub struct_property: &'a Object<'a>,
    pub string_property: &'a Object<'a>,
    pub array_property: &'a Object<'a>,
    pub map_property: &'a Object<'a>,
}

impl<'a> StaticClasses<'a> {
    pub fn new(globals: &Globals) -> Result<StaticClasses, Error> {
        let find = |class: &str| {
            globals
                .non_null_objects_iter()
                .find(|o| {
                    o.full_name(globals.names)
                        .map_or(false, |name| name == class)
                })
                .ok_or_else(|| Error::UnableToFindClass {
                    missing_class: class.to_string(),
                })
        };

        Ok(StaticClasses {
            enumeration: find("Class Core.Enum")?,
            constant: find("Class Core.Const")?,
            class: find("Class Core.Class")?,
            script_struct: find("Class Core.ScriptStruct")?,
            function: find("Class Core.Function")?,
            bool_property: find("Class Core.BoolProperty")?,
            byte_property: find("Class Core.ByteProperty")?,
            int_property: find("Class Core.IntProperty")?,
            float_property: find("Class Core.FloatProperty")?,
            object_property: find("Class Core.ObjectProperty")?,
            component_property: find("Class Core.ComponentProperty")?,
            class_property: find("Class Core.ClassProperty")?,
            interface_property: find("Class Core.InterfaceProperty")?,
            name_property: find("Class Core.NameProperty")?,
            struct_property: find("Class Core.StructProperty")?,
            string_property: find("Class Core.StrProperty")?,
            array_property: find("Class Core.ArrayProperty")?,
            map_property: find("Class Core.MapProperty")?,
        })
    }
}
