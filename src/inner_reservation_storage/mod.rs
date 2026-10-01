use aion_state::prelude::ReservationStorage;

use crate::prelude::{InnerAccessStorage, ReserverId};

pub struct InnerReservationStorage<ValueId> {
    _p: ValueId
}

impl<ValueId> ReservationStorage for InnerReservationStorage<ValueId> {
    type ReserverId = ReserverId;

    type AccessStorage = InnerAccessStorage<ValueId>;

    fn get_mut(
        &mut self, 
        key: &Self::ReserverId
    ) -> Option<&mut aion_state::prelude::Accesses<Self::AccessStorage>> {
        todo!()
    }

    fn insert(
        &mut self,
        key: Self::ReserverId,
        accesses: aion_state::prelude::Accesses<Self::AccessStorage>
    ) -> Option<aion_state::prelude::Accesses<Self::AccessStorage>> {
        todo!()
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = (
        &'a Self::ReserverId, 
        &'a aion_state::prelude::Accesses<Self::AccessStorage>
    )> 
        where Self: 'a {
        todo!()
    }
}