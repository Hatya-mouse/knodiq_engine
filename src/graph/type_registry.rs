//
// © 2025-2026 Shuntaro Kasatani
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::{collections::HashMap, fmt::Display};

#[derive(Default)]
pub struct TypeRegistry {
    name_to_id: HashMap<String, TypeID>,
    id_to_info: HashMap<TypeID, TypeInfo>,
    next_id: usize,
}

#[derive(PartialEq)]
pub struct TypeInfo {
    pub name: String,
    pub size: usize,
    pub align: usize,
}

impl TypeInfo {
    pub fn new(name: String, size: usize, align: usize) -> Self {
        Self { name, size, align }
    }
}

/// An ID used to identify a node in the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, serde::Serialize)]
pub struct TypeID(usize);

impl TypeID {
    pub fn new(val: usize) -> Self {
        Self(val)
    }
}

impl Display for TypeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TypeRegistry {
    pub fn generate_type_id(&mut self) -> TypeID {
        let id = TypeID(self.next_id);
        self.next_id += 1;
        id
    }

    pub fn register(&mut self, info: TypeInfo) -> TypeID {
        let id = self.generate_type_id();
        self.name_to_id.insert(info.name.clone(), id);
        self.id_to_info.insert(id, info);
        id
    }

    pub fn register_or_get(&mut self, info: TypeInfo) -> Option<TypeID> {
        if let Some(&id) = self.name_to_id.get(&info.name) {
            if let Some(existing_info) = self.get_info(&id)
                && existing_info == &info
            {
                return Some(id);
            }
            return None;
        }
        Some(self.register(info))
    }

    pub fn get_id(&self, name: &str) -> Option<TypeID> {
        self.name_to_id.get(name).copied()
    }

    pub fn get_info(&self, id: &TypeID) -> Option<&TypeInfo> {
        self.id_to_info.get(id)
    }
}
