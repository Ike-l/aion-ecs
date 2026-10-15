use std::{ptr::NonNull, sync::Arc};

use parking_lot::RwLock;

use crate::prelude::{EntityId, EntityMonitor, EntitySharedView, ITEMS_PER_ENTITY_STORAGE_PAGE, UniqueResource, TransmutableOwned, TransmutableShared};

pub struct EntityUniqueView<'a> {
    pages: &'a [NonNull<u8>],
    entity_monitor: Arc<RwLock<EntityMonitor>>,
    filter: Vec<EntityId>,
    layout_size: usize,
    current_index: usize,
}

impl<'a> EntityUniqueView<'a> {
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


impl<'a> Iterator for EntityUniqueView<'a> {
    type Item = (EntityId, UniqueResource);

    fn next(&mut self) -> Option<Self::Item> {
        let entity_monitor = self.entity_monitor.read();

        while self.current_index < self.filter.len() {
            let entity_id = &self.filter[self.current_index];
            self.current_index += 1;

            if entity_monitor.valid(entity_id) {
                if self.layout_size == 0 {
                    return Some((entity_id.clone(), UniqueResource::new(NonNull::<u8>::dangling().as_ptr(), None, None)));
                }
    
                let page = entity_id.index / ITEMS_PER_ENTITY_STORAGE_PAGE;
                let local = entity_id.index % ITEMS_PER_ENTITY_STORAGE_PAGE;
    
                let ptr = unsafe {
                    self.pages[page].as_ptr().add(local * self.layout_size)
                };
    
                return Some((entity_id.clone(), UniqueResource::new(ptr, None, None)));
            }
        }

        None
    }
}

impl<'a> TransmutableShared for EntityUniqueView<'a> {
    type AsShared = EntitySharedView<'a>;

    fn transmute(self) -> Self::AsShared {
        Self::AsShared::new(self.pages, self.entity_monitor, self.filter, self.layout_size)
    }
}

impl<'a> TransmutableOwned for EntityUniqueView<'a> {
    type AsOwned = ();

    fn transmute(self) -> Self::AsOwned {
        unimplemented!()
    }
}