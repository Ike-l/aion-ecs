use std::{ptr::NonNull, sync::Arc};

use parking_lot::RwLock;

use crate::prelude::{ITEMS_PER_ENTITY_STORAGE_PAGE, Resource};

pub struct StorageIterMut<'a> {
    pages: &'a [NonNull<u8>],
    alive: Arc<RwLock<Vec<bool>>>,
    layout_size: usize,
    current_index: usize,
}

impl<'a> StorageIterMut<'a> {
    pub fn new(
        pages: &'a [NonNull<u8>], 
        alive: Arc<RwLock<Vec<bool>>>,
        layout_size: usize
    ) -> Self {
        Self {
            pages,
            alive,
            layout_size,
            current_index: 0
        }
    }
}


impl<'a> Iterator for StorageIterMut<'a> {
    type Item = Resource;

    fn next(&mut self) -> Option<Self::Item> {
        let alive_guard = self.alive.read();
        while self.current_index < alive_guard.len() {
            let idx = self.current_index;
            self.current_index += 1;

            if alive_guard[idx] {
                if self.layout_size == 0 {
                    return Some(Resource::new(NonNull::<u8>::dangling().as_ptr(), None, None));
                }

                let page = idx / ITEMS_PER_ENTITY_STORAGE_PAGE;
                let local = idx % ITEMS_PER_ENTITY_STORAGE_PAGE;

                let ptr = unsafe {
                    self.pages[page].as_ptr().add(local * self.layout_size)
                };

                return Some(Resource::new(ptr, None, None));
            }
        }
        None
    }
}