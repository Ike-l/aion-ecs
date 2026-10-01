use aion_state::prelude::Registry;

use crate::prelude::{ArchetypeId, ArchetypeStorage, InnerAccessStorage, InnerBlacklistStorage, InnerControlStorage, InnerCredentialStorage, InnerReservationStorage, InnerWhitelistStorage};

pub mod archetype_storage;
pub mod archetype_id;

pub struct World {
    archetypes: Registry<
        ArchetypeStorage, 
        InnerReservationStorage<ArchetypeId>, 
        InnerAccessStorage<ArchetypeId>, 
        InnerCredentialStorage,
        InnerWhitelistStorage<ArchetypeId>,
        InnerBlacklistStorage<ArchetypeId>,
        InnerControlStorage<ArchetypeId>
    >
}