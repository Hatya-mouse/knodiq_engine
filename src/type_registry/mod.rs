mod type_id;

pub use type_id::TypeID;

use std::collections::HashMap;

pub struct TypeInfo {
    pub name: String,
    pub size: usize,
    pub align: usize,
}

pub struct TypeRegistry {
    name_to_id: HashMap<String, TypeID>,
    types: HashMap<TypeID, TypeInfo>,
    next_id: usize,
}
