use std::{ptr::NonNull, sync::Arc};

use parking_lot::RwLock;

use crate::prelude::{EntityId, EntityMonitor, ITEMS_PER_ENTITY_STORAGE_PAGE, Resource};

pub struct StorageIter<'a> {
    pages: &'a [NonNull<u8>],
    entity_monitor: Arc<RwLock<EntityMonitor>>,
    filter: Vec<EntityId>,
    layout_size: usize,
    current_index: usize,
}

impl<'a> StorageIter<'a> {
    pub fn new(
        pages: &'a [NonNull<u8>], 
        entity_monitor: Arc<RwLock<EntityMonitor>>,
        filter: Vec<EntityId>,
        layout_size: usize
    ) -> Self {
        Self {
            pages,
            entity_monitor,
            filter,
            layout_size,
            current_index: 0
        }
    }
}

impl<'a> Iterator for StorageIter<'a> {
    type Item = Resource;

    fn next(&mut self) -> Option<Self::Item> {
        let entity_monitor = self.entity_monitor.read();

        while self.current_index < self.filter.len() {
            let entity_id = &self.filter[self.current_index];
            self.current_index += 1;

            if entity_monitor.valid(entity_id) {
                if self.layout_size == 0 {
                    return Some(Resource::new(NonNull::<u8>::dangling().as_ptr(), None, None));
                }
    
                let page = entity_id.index / ITEMS_PER_ENTITY_STORAGE_PAGE;
                let local = entity_id.index % ITEMS_PER_ENTITY_STORAGE_PAGE;
    
                let ptr = unsafe {
                    self.pages[page].as_ptr().add(local * self.layout_size)
                };
    
                return Some(Resource::new(ptr, None, None));
            }
        }

        None
    }
}
