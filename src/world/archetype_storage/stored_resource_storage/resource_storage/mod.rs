use aion_state::prelude::Registry;

use crate::prelude::{EntityId, InnerAccessStorage, InnerBlacklistStorage, InnerControlStorage, InnerCredentialStorage, InnerReservationStorage, InnerWhitelistStorage, StoredEntityStorage};


pub mod entity_id;
pub mod stored_entity_storage;

pub type ResourceStorageInner = Registry<
        StoredEntityStorage,
        InnerReservationStorage<EntityId>,
        InnerAccessStorage<EntityId>,
        InnerCredentialStorage,
        InnerWhitelistStorage<EntityId>,
        InnerBlacklistStorage<EntityId>,
        InnerControlStorage<EntityId>
>;

pub struct ResourceStorage {
    inner: Arc<ResourceStorageInner>
}

