use crate::globals::Names;

mod entry;
pub use entry::Entry;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Name {
    index: u32,
    number: u32,
}

impl Name {
    pub fn entry<'n>(self, global_names: &'n Names<'n>) -> Option<&'n Entry> {
        global_names
            .get(self.index as usize)
            .copied()
            .flatten()
    }
}