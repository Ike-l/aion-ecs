use crate::prelude::EntityId;

#[derive(Default)]
pub struct EntityMonitor {
    entities: Vec<EntityId>,
    liveness: Vec<bool>
}

impl EntityMonitor {
    pub fn valid(&self, entity_id: &EntityId) -> bool {
        todo!()
        // first check index is in entities
        // then check if at index the epoch is the latest
        // then check its alive
    }
}