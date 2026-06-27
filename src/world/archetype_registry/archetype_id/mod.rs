use std::any::TypeId;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ArchetypeId {
    type_ids: Vec<TypeId>
}

impl ArchetypeId {
    pub fn insert(&mut self, type_id: TypeId) {
        self.type_ids.push(type_id);
    }   

    pub fn extract(&mut self, other: &Self) {
        self.type_ids.retain(|t| !other.type_ids.contains(t));
    } 

    pub fn is_superset(&self, other: &Self) -> bool {
        !other.type_ids.iter().any(|t| !self.type_ids.contains(t))
    }
}