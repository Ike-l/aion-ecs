use aion_state::prelude::WhitelistStorage;

use crate::prelude::Access;

pub struct InnerWhitelistStorage<Id> {
    _p: Id
}

impl<Id> WhitelistStorage for InnerWhitelistStorage<Id> {
    type Id = Id;

    type Access = Access;

    fn check_access(
        &self,
        id: &Self::Id,
        access: &Self::Access 
    ) -> bool {
        todo!()
    }

    fn allow(
        &mut self,
        id: Self::Id,
        access: Self::Access
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
    ) -> bool where <Self as WhitelistStorage>::Id: 'a {
        todo!()
    }

    fn unallow(
        &mut self,
        id: &Self::Id,
        access: &Self::Access
    ) -> bool {
        todo!()
    }
}