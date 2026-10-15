use crate::prelude::{EntityStorageRegistry, EntityStorageSharedView, ResourceId, TransmutableOwned, TransmutableShared};

pub struct EntityStorageUniqueView<'a> {
    entity_storages: std::vec::IntoIter<(&'a ResourceId, &'a mut EntityStorageRegistry)>,
}

impl<'a> EntityStorageUniqueView<'a> {
    pub fn new(entity_storages: Vec<(&'a ResourceId, &'a mut EntityStorageRegistry)>) -> Self {
        Self {
            entity_storages: entity_storages.into_iter(),
        }
    }
}

impl<'a, 'b> Iterator for EntityStorageUniqueView<'a> {
    type Item = (&'a ResourceId, &'a mut EntityStorageRegistry);

    fn next(&mut self) -> Option<Self::Item> {
        self.entity_storages.next()
    }
}

impl<'a> TransmutableShared for EntityStorageUniqueView<'a> {
    type AsShared = EntityStorageSharedView<'a>;

    fn transmute(self) -> Self::AsShared {
        Self::AsShared::new(self.entity_storages)
    }
}

impl<'a> TransmutableOwned for EntityStorageUniqueView<'a> {
    type AsOwned = EntityStorageUniqueView<'a>;

    fn transmute(self) -> Self::AsOwned {
        self
    }
}
