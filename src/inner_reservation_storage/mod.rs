use std::collections::HashMap;

use aion_state::prelude::{Accesses, ReservationStorage};

use crate::prelude::{InnerAccessStorage, ReserverId};

pub struct InnerReservationStorage<ValueId> {
    inner: HashMap<ReserverId, Accesses<InnerAccessStorage<ValueId>>>
}

impl<ValueId> ReservationStorage for InnerReservationStorage<ValueId> {
    type ReserverId = ReserverId;
    type AccessStorage = InnerAccessStorage<ValueId>;

    fn get_mut(
        &mut self, 
        key: &Self::ReserverId
    ) -> Option<&mut aion_state::prelude::Accesses<Self::AccessStorage>> {
        self.inner.get_mut(key)
    }

    fn insert(
        &mut self,
        key: Self::ReserverId,
        accesses: aion_state::prelude::Accesses<Self::AccessStorage>
    ) -> Option<aion_state::prelude::Accesses<Self::AccessStorage>> {
        self.inner.insert(key, accesses)
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = (
        &'a Self::ReserverId, 
        &'a aion_state::prelude::Accesses<Self::AccessStorage>
    )> 
        where Self: 'a {
        self.inner.iter()
    }
}