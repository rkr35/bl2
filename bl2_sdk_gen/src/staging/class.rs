use bl2_core::{
    game::{BoolProperty, cast, Class as GameClass, Field, Object, Property,
        Struct},
    globals::Globals,
};

use core::cmp::Ordering;
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

    #[error("Encountered Default__ class \"{0}\".")]
    DefaultClass(String),
}

pub struct Class<'a> {
    pub name: &'a str,
    pub full_name: String,
    pub size: u32,
    pub inherited_size: u32,
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
            return Err(Error::DefaultClass(name.to_owned()));
        }

        let full_name = class
            .full_name(globals.names)
            .ok_or(Error::UnableToGenerateFullName)?;

        let maybe_parent = class
            .super_field
            .filter(|parent| parent.ne(class));

        let inherited_size = maybe_parent
            .map_or(0_u32, |parent| parent.property_size.into());
        
        let children = iter::successors(
            class.children,
            |c| c.next.map(|f| unsafe { cast::<Property>(f) })
        );

        let children = {
            let mut c: Vec<_> = children
                .filter(|c| c.element_size > 0)
                .filter(|c| !c.is(static_classes.script_struct))
                .filter(|c| !c.is(static_classes.function))
                .filter(|c| !c.is(static_classes.enumeration))
                .filter(|c| !c.is(static_classes.constant))
                .filter(|c| c.offset >= inherited_size)
                .collect();

            c.sort_by(|p, q|
                p
                    .offset
                    .cmp(&q.offset)
                    .then_with(|| {
                        let to_bool = {
                            type O<'a> = &'a Object<'a>;
                            let is = |r: O| r.is(static_classes.bool_property);
                            let to = |r| unsafe { cast::<BoolProperty>(r) };
                            move |r| if is(r) { Some(to(r)) } else { None }
                        };

                        match [to_bool(p), to_bool(q)] {
                            [Some(p), Some(q)] => p.bitmask.cmp(&q.bitmask),
                            _ => Ordering::Equal,
                        }
                    })
            );

            c
        };

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
