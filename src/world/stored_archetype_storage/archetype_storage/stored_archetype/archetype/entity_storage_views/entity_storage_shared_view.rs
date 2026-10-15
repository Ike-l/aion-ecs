use crate::prelude::{EntityStorageRegistry, ResourceId};

pub struct EntityStorageSharedView<'a> {
    entity_storages: std::vec::IntoIter<(&'a ResourceId, &'a EntityStorageRegistry)>,
}

impl<'a> EntityStorageSharedView<'a> {
    pub fn new(entity_storages: std::vec::IntoIter<(&'a ResourceId, &'a mut EntityStorageRegistry)>) -> Self {
        Self {
            entity_storages: entity_storages
                .map(|(resource_id, entity_storage)| (resource_id, &*entity_storage))
                .collect::<Vec<_>>()
                .into_iter(),
        }
    }
}

impl<'a, 'b> Iterator for EntityStorageSharedView<'a> {
    type Item = (&'a ResourceId, &'a EntityStorageRegistry);

    fn next(&mut self) -> Option<Self::Item> {
        self.entity_storages.next()
    }
}