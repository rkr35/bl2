use bl2_core::game::Object;

use std::collections::HashMap;

mod subpackage;
pub use subpackage::SubPackage;

#[derive(Default)]
pub struct Package<'a> {
    pub subpackages: HashMap<&'a Object<'a>, SubPackage<'a>>,
}