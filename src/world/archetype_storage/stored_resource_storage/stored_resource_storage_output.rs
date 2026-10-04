use aion_state::prelude::{RegistryReleaseAccess, ReleasingResult};

use crate::prelude::{Access, AccessResult, ResourceStorageInner, StoredResource, TransmutableShared};

pub struct StoredResourceStorageOutput<'a> { pub result: ReleasingResult<'a, &'a mut StoredResource, AccessResult<&'a mut StoredResource>, ResourceStorageInner> }

impl<'a> StoredResourceStorageOutput<'a> {
    pub fn new(result: ReleasingResult<'a, &'a mut StoredResource, AccessResult<&'a mut StoredResource>, ResourceStorageInner>) -> Self {
        Self { result }
    }
}

impl TransmutableShared for StoredResourceStorageOutput<'_> {
    type AsShared = Self;

    fn transmute(self) -> Self::AsShared {
        let Some(releaser) = self.result.access_releaser() else { unimplemented!() };
        let Some(release_input) = self.result.access_release_input() else { unimplemented!() };
        
        assert!(release_input.access == Access::Unique);

        unsafe { releaser.release_access(&RegistryReleaseAccess {
            resource_id: &release_input.resource_id,
            access: &Access::UniqueToShared,
        }) };

        self
    }
}