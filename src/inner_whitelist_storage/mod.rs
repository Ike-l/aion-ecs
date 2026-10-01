use std::{collections::HashMap, hash::Hash};

use aion_state::prelude::WhitelistStorage;

use crate::prelude::Access;

pub struct InnerWhitelistStorage<Id> {
    inner: HashMap<Id, Vec<Access>>
}

impl<Id> WhitelistStorage for InnerWhitelistStorage<Id> 
    where Id: PartialEq + Eq + Hash
{
    type Id = Id;
    type Access = Access;

    fn check_access(
        &self,
        id: &Self::Id,
        access: &Self::Access 
    ) -> bool {
        let Some(allowed_accesses) = self.inner.get(id) else { return false };
        allowed_accesses.iter().any(|allowed_access| allowed_access == access)
    }

    fn allow(
        &mut self,
        id: Self::Id,
        access: Self::Access
    ) -> bool {
        self.inner.entry(id).or_default().push(access);

        true
    }

    fn release(
        &mut self,
        id: &Self::Id
    ) -> bool {
        self.inner.remove(id).is_some()
    }

    fn release_all<'a>(
        &mut self,
        mut ids: impl Iterator<Item = &'a Self::Id>
    ) -> bool where <Self as WhitelistStorage>::Id: 'a {
        !ids.any(|resource_id| !self.release(resource_id))
    }

    fn unallow(
        &mut self,
        id: &Self::Id,
        access: &Self::Access
    ) -> bool {
        let Some(allowed_accesses) = self.inner.get_mut(id) else { return false };

        let Some(position) = allowed_accesses.iter().position(|allowed_access| allowed_access == access) else { return false };

        allowed_accesses.remove(position);

        true
    }
}