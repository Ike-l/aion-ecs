use crate::prelude::ArchetypeStorage;

pub mod archetype_storage;
pub mod archetype_id;

pub struct StoredArchetypeStorage {
    archetype_storage: ArchetypeStorage
}