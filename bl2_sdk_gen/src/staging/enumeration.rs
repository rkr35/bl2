use bl2_core::{
    game::Enum,
    globals::Globals,
};

#[derive(Debug)]
pub struct Enumeration<'a> {
    pub name: &'a str,
    pub full_name: String,
    pub variants: Vec<&'a str>,
}

impl<'a> Enumeration<'a> {
    pub fn from<'n>(enumeration: &Enum, globals: &'n Globals) -> Option<Enumeration<'n>> {
        let name = enumeration.name(globals.names)?;

        if name.contains("Default__") {
            return None;
        }

        Some(Enumeration {
            name,
            full_name: enumeration.full_name(globals.names)?,
            variants: enumeration.variants(globals.names)?,
        })
    }
}
