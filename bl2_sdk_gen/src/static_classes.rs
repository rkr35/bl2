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
        })
    }
}
