use std::sync::Arc;

use aion_program::prelude::AutoRegistry;
use aion_state::prelude::{RegistryReleasingAcquireAccess, Releaser};

use crate::prelude::{ArchetypeAccess, ArchetypeId, StoredArchetype};

pub mod archetype_id;
pub mod stored_archetype;
pub mod archetype_access;

#[derive(Default)]
pub struct ArchetypeRegistry {
    registry: Arc<AutoRegistry<ArchetypeId, StoredArchetype, ArchetypeAccess>>
}

impl ArchetypeRegistry {
    pub fn find_at_least(&self, archetype_id: &ArchetypeId) {
        let archetypes = self.registry.keys();
        let using_archetypes = archetypes.filter(|archetype| {
            archetype.is_superset(archetype_id)
        });

        let archetypes = using_archetypes.filter_map(|archetype| {
            <AutoRegistry<ArchetypeId, StoredArchetype, ArchetypeAccess> as Releaser>::acquire_access(&self.registry, RegistryReleasingAcquireAccess {
                user_details: None,
                resource_id: archetype,
                access: ArchetypeAccess::Shared(1),
                password: None
            }).ok()
        });

        for a in archetypes {
            let b = a.as_ref();
            let c = b.unwrap();
        }
    }
}