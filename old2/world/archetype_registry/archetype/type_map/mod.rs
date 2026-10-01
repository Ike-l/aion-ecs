use aion_state::prelude::Registry;

use crate::prelude::EntityId;

pub mod entity_id;

pub struct TypeMap<T> {
    entity_map: Registry<EntityId, T>
}