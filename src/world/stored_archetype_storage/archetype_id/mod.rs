use crate::prelude::ResourceId;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ArchetypeId {
    resource_ids: Vec<ResourceId>
}

impl ArchetypeId {
    pub fn contains(&self, resource_id: &ResourceId) -> bool {
        self.resource_ids.contains(resource_id)
    }
}