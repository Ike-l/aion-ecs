use std::collections::HashMap;

use aion_state::prelude::CredentialStorage;

use crate::prelude::{Password, ReserverId};

pub struct InnerCredentialStorage {
    inner: HashMap<ReserverId, Password>
}

impl CredentialStorage for InnerCredentialStorage {
    type Id = ReserverId;
    type Password = Password;

    fn verify(
        &self,
        id: &Self::Id, 
        password: &Self::Password
    ) -> bool {
        self.inner.get(id).is_some_and(|registered_password| registered_password == password)
    }

    fn registered(&self) -> impl Iterator<Item = &Self::Id> {
        self.inner.keys()
    }

    fn register(
        &mut self,
        id: Self::Id,
        password: Self::Password
    ) -> bool {
        if self.inner.contains_key(&id) {
            return false
        }
        
        self.inner.insert(id, password).is_none()
    }

    fn update_password(
        &mut self,
        id: &Self::Id,
        new_password: Self::Password
    ) -> bool {
        let Some(old_password) = self.inner.get_mut(id) else { return false };

        *old_password = new_password;

        true
    }

    fn unregister(
        &mut self,
        id: &Self::Id
    ) -> bool {
        self.inner.remove(id).is_some()
    }
}