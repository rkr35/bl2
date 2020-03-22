use bl2_core::{
    game::Class as GameClass,
    globals::Globals,
};


pub struct Class<'a> {
    pub name: &'a str,
}

impl<'a> Class<'a> {
    pub fn from<'n>(class: &GameClass, globals: &'n Globals) -> Option<Class<'n>> {
        let name = class.name(globals.names)?;

        if name.contains("Default__") {
            return None;
        }

        Some(Class {
            name,
        })
    }
}
