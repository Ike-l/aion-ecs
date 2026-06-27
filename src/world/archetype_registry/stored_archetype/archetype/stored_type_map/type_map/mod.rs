use aion_program::prelude::AutoRegistry;

use crate::prelude::{EntityAccess, EntityId, StoredType};

pub mod entity_id;
pub mod stored_type;
pub mod entity_access;

pub type TypeMap = AutoRegistry<EntityId, StoredType, EntityAccess>;