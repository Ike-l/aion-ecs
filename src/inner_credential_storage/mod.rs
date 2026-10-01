use aion_state::prelude::CredentialStorage;

use crate::prelude::{Password, ReserverId};

pub struct InnerCredentialStorage {

}

impl CredentialStorage for InnerCredentialStorage {
    type Id = ReserverId;

    type Password = Password;

    fn verify(
        &self,
        id: &Self::Id, 
        password: &Self::Password
    ) -> bool {
        todo!()
    }

    fn registered(&self) -> impl Iterator<Item = &Self::Id> {
        todo!()
    }

    fn register(
        &mut self,
        id: Self::Id,
        password: Self::Password
    ) -> bool {
        todo!()
    }

    fn update_password(
        &mut self,
        id: &Self::Id,
        new_password: Self::Password
    ) -> bool {
        todo!()
    }

    fn unregister(
        &mut self,
        id: &Self::Id
    ) -> bool {
        todo!()
    }
}