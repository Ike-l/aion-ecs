use std::{alloc::{Layout, alloc, dealloc}, ptr::{self, NonNull}, sync::Arc};

use parking_lot::RwLock;

use crate::prelude::{Resource, StorageIter, StorageIterMut, StoredResource};

pub mod stored_resource;
pub mod storage_iter;
pub mod storage_iter_mut;

pub const ITEMS_PER_ENTITY_STORAGE_PAGE: usize = 1024;

pub struct EntityMap {
    pages: Vec<NonNull<u8>>,
    layout: Layout,
    drop_fn: unsafe fn(*mut u8),
    write_fn: unsafe fn(*const u8, *mut u8),
    alive: Arc<RwLock<Vec<bool>>>,
}

impl EntityMap {
    pub fn new<T: 'static>(alive: Arc<RwLock<Vec<bool>>>) -> Self {
        Self {
            pages: Vec::new(),
            layout: Layout::new::<T>(),
            drop_fn: |ptr| unsafe { ptr::drop_in_place(ptr as *mut T); },
            write_fn: |src, dst| unsafe {
                let val = ptr::read(src as *const T);
                ptr::write(dst as *mut T, val);
            },
            alive
        }
    }

    /// Allocates pages until `index` exists
    fn create_capacity(&mut self, index: usize) {
        let required_pages = (index / ITEMS_PER_ENTITY_STORAGE_PAGE) + 1;

        while self.pages.len() < required_pages {
            if self.layout.size() == 0 {
                self.pages.push(NonNull::dangling());
            } else {
                let page_layout = Layout::from_size_align(
                    self.layout.size() * ITEMS_PER_ENTITY_STORAGE_PAGE, 
                    self.layout.align()
                ).unwrap();

                let ptr = unsafe { alloc(page_layout) };
                self.pages.push(NonNull::new(ptr).expect("Memory Allocation Failed"));
            }
        }
    }

    /// Calculates where the index exists and returns the pointer to that element
    /// 
    /// Safety: 
    /// * `index` is valid
    unsafe fn get_raw_ptr_mut(&self, index: usize) -> *mut u8 {
        let page = index / ITEMS_PER_ENTITY_STORAGE_PAGE;
        let local = index % ITEMS_PER_ENTITY_STORAGE_PAGE;
        let page_ptr = self.pages[page].as_ptr();
        unsafe { page_ptr.add(local * self.layout.size()) }
    }

    /// Drops the piece of memory at `index`
    /// 
    /// Safety: 
    /// * Caller must guarantee the slot is currently alive and active.
    pub unsafe fn drop_at(&mut self, index: usize) {
        if self.layout.size() > 0 {
            let ptr = unsafe { self.get_raw_ptr_mut(index) };
            unsafe { (self.drop_fn)(ptr) };
        }
    }

    /// Writes `value` to `index`
    pub unsafe fn write(&mut self, index: usize, stored_resource: StoredResource) {
        self.create_capacity(index);

        if self.layout.size() > 0 {
            let dst = unsafe { self.get_raw_ptr_mut(index) };
            unsafe { (self.write_fn)(stored_resource.ptr(), dst) };

            if stored_resource.layout().size() > 0 {
                unsafe { dealloc(stored_resource.ptr(), *stored_resource.layout()) };
            }
        }

        std::mem::forget(stored_resource);
    }

    /// Safety:
    /// * `index` is valid
    pub unsafe fn get_mut(&mut self, index: usize) -> Option<Resource> {
        if !self.is_alive(index) {
            return None
        }

        let ptr = unsafe { self.get_raw_ptr_mut(index) };
        Some(Resource::new(ptr, None, None))
    }

    pub fn is_alive(&self, index: usize) -> bool {
        let alive = self.alive.read();
        !(index >= alive.len() || !alive[index])
    }

    pub fn iter(&self) -> StorageIter<'_> {
        StorageIter::new(
            &self.pages,
            Arc::clone(&self.alive),
            self.layout.size(),
        )
    }

    pub fn iter_mut(&mut self) -> StorageIterMut<'_> {
        StorageIterMut::new(
            &self.pages,
            Arc::clone(&self.alive),
            self.layout.size(),
        )
    }
}

impl Drop for EntityMap {
    fn drop(&mut self) {
        unsafe {
            if self.layout.size() > 0 {
                let page_layout = Layout::from_size_align(
                    self.layout.size() * ITEMS_PER_ENTITY_STORAGE_PAGE, 
                    self.layout.align()
                ).unwrap();

                for page in &self.pages {
                    dealloc(page.as_ptr(), page_layout);
                }
            }
        }
    }
}