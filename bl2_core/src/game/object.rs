use core::hash::{Hash, Hasher};
use core::iter;
use core::ptr;
use crate::game::{Entry, Name};
use crate::globals::Names;
use super::Struct;

/*
class UObject
{
public:
    FPointer		VTableObject;
    char			UnknownData00[0x1C];
    uint32_t		InternalIndex;
    char			UnknownData01[0x04];
    UObject*		Outer;
    FName			Name;
    UObject*		Class;
    char			UnknownData02[0x04];
};

*/

#[repr(C)]
pub struct Object<'a> {
    vtable: usize,
    pad0: [u8; 0x1c],
    index: u32,
    pad1: [u8; 0x4],
    outer: Option<&'a Object<'a>>,
    name: Name,
    class: Option<&'a Struct<'a>>,
    pad2: [u8; 0x4],
}

impl<'a> Object<'a> {
    pub fn name<'n>(&self, global_names: &'n Names) -> Option<&'n str> {
        self.entry(global_names)?.to_str().ok()
    }

    pub fn entry<'n>(&self, global_names: &'n Names) -> Option<&'n Entry> {
        self.name.entry(global_names)
    }

    pub fn full_name<'n>(&self, global_names: &'n Names) -> String {
        let outers = {
            let mut v: Vec<_> = self
                .outer_iter()
                .map(|o| o.name(global_names).unwrap_or("!OUTER_UNKNOWN!"))
                .collect();
            v.reverse();
            v.join(".")
        };

        let class = self
            .class
            .and_then(|c| c.name(global_names))
            .unwrap_or("!CLASS_UNKNOWN!");
            
        let self_name = self
            .name(global_names)
            .unwrap_or("!SELF_UNKNOWN!");

        if outers.is_empty() {
            format!("{} {}", class, self_name)
        } else {
            format!("{} {}.{}", class, outers, self_name)
        }
    }
    
    pub fn package(&self) -> Option<&Object> {
        self.outer_iter().last()
    }

    fn outer_iter(&self) -> impl Iterator<Item = &Object> {
        iter::successors(self.outer, |o| o.outer)
    }

    pub fn class_iter(&self) -> impl Iterator<Item = &Struct> {
        iter::successors(self.class, |o| o.super_field)
    }

    pub fn is(&self, class: &Object) -> bool {
        self.class_iter().any(|c| c.eq(class))
    }
}

impl<'a> PartialEq for Object<'a> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}

impl<'a> Eq for Object<'a> {}

impl<'a> Hash for Object<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let pointer = self as *const _ as usize;
        pointer.hash(state);
    }
}