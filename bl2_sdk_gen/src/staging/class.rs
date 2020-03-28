use bl2_core::{
    game::{cast, Class as GameClass, Field, Property, Struct},
    globals::Globals,
};

use core::convert::TryInto;
use core::iter;
use crate::StaticClasses;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to generate name.")]
    UnableToGenerateName,

    #[error("Unable to generate full name.")]
    UnableToGenerateFullName,

    #[error("Encountered Default__ class.")]
    DefaultClass,

    #[error("Inherited size ({0}_u32) does not losslessly cast into usize.")]
    InheritedSizeLossyCast(u32),
}

pub struct Class<'a> {
    pub name: &'a str,
    pub full_name: String,
    pub size: usize,
    pub inherited_size: usize,
    pub parent: Option<&'a Struct<'a>>,
}

impl<'a> Class<'a> {
    pub fn from<'n>(
        class: &GameClass<'n>,
        globals: &'n Globals,
        static_classes: &StaticClasses
    ) -> Result<Class<'n>, Error> {

        let name = class
            .name(globals.names)
            .ok_or(Error::UnableToGenerateName)?;

        if name.contains("Default__") {
            return Err(Error::DefaultClass);
        }

        let maybe_parent: Option<&Struct> = class
            .super_field
            .filter(|parent| parent.ne(class));

        let inherited_size = maybe_parent
            .map_or(0_u32, |parent| parent.property_size.into());
        
        let children = iter::successors(
            class.children,
            |c| c.next.map(|f| unsafe { cast::<Property>(f) })
        );

        let children = children
            .filter(|c| c.element_size > 0)
            .filter(|c| !c.is(static_classes.script_struct))
            .filter(|c| !c.is(static_classes.function))
            .filter(|c| !c.is(static_classes.enumeration))
            .filter(|c| !c.is(static_classes.constant))
            .filter(|c| c.offset >= inherited_size)
            ;

        let full_name = class
            .full_name(globals.names)
            .ok_or(Error::UnableToGenerateFullName)?;

        let inherited_size = inherited_size
            .try_into()
            .map_err(|_| Error::InheritedSizeLossyCast(inherited_size))?;

        Ok(Class {
            name,
            full_name,
            size: class.property_size.into(),
            inherited_size,
            parent: maybe_parent,
        })
    }
}
