use aion_state::prelude::BlacklistStorage;

use crate::prelude::{Access, Password};

pub struct InnerBlacklistStorage<Id> {
    _p: Id
}

impl<Id> BlacklistStorage for InnerBlacklistStorage<Id> {
    type Id = Id;

    type Access = Access;

    type Password = Password;

    fn check_access(
        &self,
        id: &Self::Id,
        access: &Self::Access,
        password: &Self::Password
    ) -> bool {
        todo!()
    }

    fn allow(
        &mut self,
        id: Self::Id,
        access: Self::Access
    ) -> Option<Self::Password> {
        todo!()
    }

    fn unallow(
        &mut self,
        id: &Self::Id,
        access: &Self::Access
    ) -> bool {
        todo!()
    }

    fn release(
        &mut self,
        id: &Self::Id
    ) -> bool {
        todo!()
    }

    fn release_all<'a>(
        &mut self,
        ids: impl Iterator<Item = &'a Self::Id>
    ) -> bool where <Self as BlacklistStorage>::Id: 'a {
        todo!()
    }
}