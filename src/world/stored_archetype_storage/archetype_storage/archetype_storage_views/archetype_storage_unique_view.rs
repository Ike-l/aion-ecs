use std::sync::Arc;

use crate::prelude::{ArchetypeId, ArchetypeStorageSharedView, StoredArchetypeRegistry, TransmutableOwned, TransmutableShared};

pub struct ArchetypeStorageUniqueView<'a> {
    archetypes: std::vec::IntoIter<(&'a ArchetypeId, &'a mut Arc<StoredArchetypeRegistry>)>,
}

impl<'a> ArchetypeStorageUniqueView<'a> {
    pub fn new(archetypes: Vec<(&'a ArchetypeId, &'a mut Arc<StoredArchetypeRegistry>)>) -> Self {
        Self {
            archetypes: archetypes.into_iter(),
        }
    }
}

impl<'a, 'b> Iterator for ArchetypeStorageUniqueView<'a> {
    type Item = (&'a ArchetypeId, &'a mut Arc<StoredArchetypeRegistry>);

    fn next(&mut self) -> Option<Self::Item> {
        self.archetypes.next()
    }
}

impl<'a> TransmutableShared for ArchetypeStorageUniqueView<'a> {
    type AsShared = ArchetypeStorageSharedView<'a>;

    fn transmute(self) -> Self::AsShared {
        Self::AsShared::new(self.archetypes)
    }
}

impl<'a> TransmutableOwned for ArchetypeStorageUniqueView<'a> {
    type AsOwned = ArchetypeStorageUniqueView<'a>;

    fn transmute(self) -> Self::AsOwned {
        self
    }
}
