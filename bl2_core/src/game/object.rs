use core::hash::{Hash, Hasher};
use core::iter;
use core::ptr;
use crate::game::{Entry, Name};
use crate::globals::Names;

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
    class: Option<&'a Object<'a>>,
    pad2: [u8; 0x4],
}

impl<'a> Object<'a> {
    pub fn name<'n>(&self, global_names: &'n Names<'n>) -> Option<&'n str> {
		self.entry(global_names)?.to_str().ok()
	}

    pub fn entry<'n>(&self, global_names: &'n Names<'n>) -> Option<&'n Entry> {
        self.name.entry(global_names)
    }

    pub fn full_name(&self, _global_names: &Names) -> String {
		todo!();
        /*
        	if (GetClass().IsValid())
	{
		std::string temp;

		for (auto outer = GetOuter(); outer.IsValid(); outer = outer.GetOuter())
		{
			temp = outer.GetName() + "." + temp;
		}

		std::string name = GetClass().GetName();
		name += " ";
		name += temp;
		name += GetName();

		return name;
	}

        */
	}
	
	pub fn get_package(&self) -> Option<&Object> {
		self.outer_iter().last()
	}

	fn outer_iter(&self) -> impl Iterator<Item = &Object> {
		let mut current = self;
		iter::from_fn(move || Some({
			current = current.outer?;
			current
		}))
	}

	pub fn super_iter(&self) -> impl Iterator<Item = &Object> {
		let mut current = self;
		iter::from_fn(move || Some({
			current = current.class?;
			current
		}))
	}

	pub fn is(&self, class: &Object) -> bool {
		self.super_iter().any(|c| ptr::eq(c, class))
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