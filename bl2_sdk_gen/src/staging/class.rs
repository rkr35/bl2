use bl2_core::{
    game::{
        Array,
        ArrayProperty,
        BoolProperty,
        ByteProperty,
        cast,
        Class as GameClass,
        ClassProperty,
        Field,
        FString,
        InterfaceProperty,
        Name,
        Object,
        ObjectProperty,
        Property,
        ScriptInterface,
        Struct,
        StructProperty,
    },
    globals::Globals,
};

use core::cmp::Ordering;
use core::convert::TryInto;
use core::iter;
use core::mem;
use crate::StaticClasses;
use std::borrow::Cow;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to generate name.")]
    UnableToGenerateName,

    #[error("Unable to generate full name.")]
    UnableToGenerateFullName,

    #[error("Encountered Default__ class \"{0}\".")]
    DefaultClass(String),

    #[error("Member error: {source}")]
    Member {
        #[from]
        source: MemberError,
    }
}

#[derive(Debug)]
pub enum MemberErrorKind {
    BadU32ToUsizeCast(u32),

    BadFieldName,

    BadEnumName,
    
    ClassIsNull,
    BadClassName,

    MetaClassIsNull,
    BadMetaClassName,

    InterfaceClassIsNull,
    BadInterfaceClassName,

    UnknownProperty,

    StructIsNull,
    BadStructName,

    ArrayInnerTypeIsNull,
    BadArrayInnerTypeName,
}

#[derive(Error, Debug)]
#[error("{kind:?} at offset {offset}")]
pub struct MemberError {
    kind: MemberErrorKind,
    offset: u32,
}

pub struct Class<'a> {
    pub name: &'a str,
    pub full_name: String,
    pub size: u32,
    pub inherited_size: u32,
    pub parent: Option<&'a Struct<'a>>,
    pub members: Vec<Member<'a>>,
}

pub enum Kind<'a> {
    Array { inner: Box<Kind<'a>>, },
    Bool,
    Byte,
    Class { name: &'a str, },
    Enum { name: &'a str, },
    Float,
    Int,
    Interface { name: &'a str, },
    Name,
    Object { name: &'a str },
    Padding,
    String,
    Struct { name: &'a str },
}

pub struct Member<'a> {
    kind: Kind<'a>,
    name: Cow<'a, str>,
    offset: u32,
    size: u32,
    comment: &'static str,
}

fn get_info<'n>(prop: &Property, globals: &'n Globals, static_classes: &StaticClasses) -> Result<(Kind<'n>, usize), MemberErrorKind> {
    use MemberErrorKind::*;

    if prop.is(static_classes.byte_property) {
        let p = unsafe { cast::<ByteProperty>(prop) };
        if let Some(e) = p.r#enum {
            let name = e.name(globals.names).ok_or(BadEnumName)?;
            Ok((Kind::Enum { name }, 1))
        } else {
            Ok((Kind::Byte, 1))
        }
    } 
    
    else if prop.is(static_classes.int_property) {
        Ok((Kind::Int, 4))
    } 
    
    else if prop.is(static_classes.float_property) {
        Ok((Kind::Float, 4))
    } 
    
    else if prop.is(static_classes.bool_property) {
        Ok((Kind::Bool, 4))
    } 
    
    else if prop.is(static_classes.object_property) || prop.is(static_classes.component_property) {
        let p = unsafe { cast::<ObjectProperty>(prop) };
        let c = p.class.ok_or(ClassIsNull)?;
        let name = c.name(globals.names).ok_or(BadClassName)?;
        Ok((Kind::Object { name }, mem::size_of::<usize>()))
    } 
    
    else if prop.is(static_classes.class_property) {
        let p = unsafe { cast::<ClassProperty>(prop) };
        let mc = p.meta_class.ok_or(MetaClassIsNull)?;
        let name = mc.name(globals.names).ok_or(BadMetaClassName)?;
        Ok((Kind::Class { name }, mem::size_of::<usize>()))
    } 
    
    else if prop.is(static_classes.interface_property) {
        let p = unsafe { cast::<InterfaceProperty>(prop) };
        let c = p.class.ok_or(InterfaceClassIsNull)?;
        let name = c.name(globals.names).ok_or(BadInterfaceClassName)?;
        Ok((Kind::Interface { name }, mem::size_of::<ScriptInterface>()))
    } 
    
    else if prop.is(static_classes.name_property) {
        Ok((Kind::Name, mem::size_of::<Name>()))
    }
    
    else if prop.is(static_classes.struct_property) {
        let p = unsafe { cast::<StructProperty>(prop) };
        let s = p.r#struct.ok_or(StructIsNull)?;
        let name = s.name(globals.names).ok_or(BadStructName)?;
        let size = prop.element_size;
        let size = size.try_into().map_err(|_| BadU32ToUsizeCast(size))?;
        Ok((Kind::Struct { name }, size))
    }

    else if prop.is(static_classes.string_property) {
        Ok((Kind::String, mem::size_of::<FString>()))
    }
    
    else if prop.is(static_classes.array_property) {
        let p = unsafe { cast::<ArrayProperty>(prop) };
        let inner = p.inner.ok_or(ArrayInnerTypeIsNull)?;
        let (inner, _) = get_info(inner, globals, static_classes)?;
        let inner = Box::new(inner);
        Ok((Kind::Array { inner }, mem::size_of::<Array<usize>>()))
    }

    else if prop.is(static_classes.map_property) {
        let p = unsafe { cast::<ArrayProperty>(prop) };
    }

    else {
        Err(MemberErrorKind::UnknownProperty)
    }
}

fn get_members<'n>(children: Vec<&Property>, mut offset: u32, globals: &'n Globals, static_classes: &StaticClasses) -> Result<Vec<Member<'n>>, MemberError> {
    let mut previous_bitfield: Option<()> = None;
    let mut num_paddings = 0;
    let mut members = vec![];
    
    for child in children {
        if offset < child.offset {
            previous_bitfield = None;
            members.push(Member {
                kind: Kind::Padding,
                name: Cow::Owned(format!("pad{}", num_paddings)),
                offset,
                size: child.offset - offset,
                comment: "MISSED OFFSET",
            });
            num_paddings += 1;
        }

        let name = child.name(globals.names).ok_or(MemberError {
            kind: MemberErrorKind::BadFieldName,
            offset,
        });


        let (kind, size) = get_info(child, globals, static_classes).map_err(|kind| MemberError {
            kind,
            offset,
        })?;
    }

    Ok(members)
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

        Ok(Class {
            name,
            full_name,
            size: class.property_size.into(),
            inherited_size,
            parent: maybe_parent,
            members: get_members(children, inherited_size, globals, static_classes)?,
        })
    }
}