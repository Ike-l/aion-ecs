use crate::prelude::EntityId;

#[derive(Default)]
pub struct EntityMonitor {
    entities: Vec<(EntityId, bool)>,
}

impl EntityMonitor {
    pub fn valid(&self, entity_id: &EntityId) -> bool {
        if entity_id.index >= self.entities.len() {
            return false
        }

        let (current_entity_id, alive) = &self.entities[entity_id.index];

        if current_entity_id.epoch != entity_id.epoch {
            return false
        }

        *alive
    }
}