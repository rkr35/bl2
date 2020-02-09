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
    pub fn name<'n>(&self, global_names: &'n Names<'n>) -> Option<&'n Entry> {
        self.name.entry(global_names)
    }

    pub fn full_name(&self, global_names: &Names) -> String {
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
}