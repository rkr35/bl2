use bl2_core::{
    game::Enum as GameEnum,
    globals::Globals,
};

#[derive(Debug)]
pub struct Enum<'a> {
    pub name: &'a str,
    pub full_name: String,
    pub variants: Vec<&'a str>,
}

impl<'a> Enum<'a> {
    pub fn from<'n>(enumeration: &GameEnum, globals: &'n Globals) -> Option<Enum<'n>> {
        let name = enumeration.name(globals.names)?;

        if name.contains("Default__") {
            return None;
        }

        Some(Enum {
            name,
            full_name: enumeration.full_name(globals.names)?,
            variants: enumeration.variants(globals.names)?,
        })
    }
}
