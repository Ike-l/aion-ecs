use crate::prelude::{ArchetypeId, StoredArchetypeRegistry};

pub struct ArchetypeStorageSharedView<'a> {
    archetypes: std::vec::IntoIter<(&'a ArchetypeId, &'a StoredArchetypeRegistry)>,
}

impl<'a> ArchetypeStorageSharedView<'a> {
    pub fn new(archetypes: std::vec::IntoIter<(&'a ArchetypeId, &'a mut StoredArchetypeRegistry)>) -> Self {
        Self {
            archetypes: archetypes
                .map(|(archetype_id, archetype)| (archetype_id, &*archetype))
                .collect::<Vec<_>>()
                .into_iter(),
        }
    }
}

impl<'a, 'b> Iterator for ArchetypeStorageSharedView<'a> {
    type Item = (&'a ArchetypeId, &'a StoredArchetypeRegistry);

    fn next(&mut self) -> Option<Self::Item> {
        self.archetypes.next()
    }
}