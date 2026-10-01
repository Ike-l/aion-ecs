use aion_state::prelude::Registry;

use crate::prelude::{InnerAccessStorage, InnerBlacklistStorage, InnerControlStorage, InnerCredentialStorage, InnerReservationStorage, InnerWhitelistStorage};

pub struct World {
    archetypes: Registry<
        InnerStorage, 
        InnerReservationStorage<ValueId>, 
        InnerAccessStorage<ValueId>, 
        InnerCredentialStorage,
        InnerWhitelistStorage<ValueId>,
        InnerBlacklistStorage<ValueId>,
        InnerControlStorage<ValueId>
    >
}