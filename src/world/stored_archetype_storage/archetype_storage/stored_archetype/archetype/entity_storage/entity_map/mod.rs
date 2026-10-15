use std::{alloc::{Layout, alloc, dealloc}, ptr::{self, NonNull}, sync::Arc};

use parking_lot::RwLock;

use crate::prelude::{EntityId, EntityMonitor, Resource, EntityUniqueView, StoredResource};

pub mod stored_resource;
pub mod entity_views;

pub const ITEMS_PER_ENTITY_STORAGE_PAGE: usize = 1024;

pub struct EntityMap {
    pages: Vec<NonNull<u8>>,
    layout: Layout,
    drop_fn: unsafe fn(*mut u8),
    write_fn: unsafe fn(*const u8, *mut u8),
    entity_monitor: Arc<RwLock<EntityMonitor>>,
}

impl EntityMap {
    pub fn new<T: 'static>(entity_monitor: Arc<RwLock<EntityMonitor>>) -> Self {
        Self {
            pages: Vec::new(),
            layout: Layout::new::<T>(),
            drop_fn: |ptr| unsafe { ptr::drop_in_place(ptr as *mut T); },
            write_fn: |src, dst| unsafe {
                let val = ptr::read(src as *const T);
                ptr::write(dst as *mut T, val);
            },
            entity_monitor
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

    pub fn valid(&self, entity_id: &EntityId) -> bool {
        self.entity_monitor.read().valid(entity_id)
    }

    pub fn remove(&mut self, entity_id: &EntityId) -> Option<StoredResource> {
        if !self.entity_monitor.read().valid(entity_id) {
            return None;
        }

        let layout = self.layout;

        let raw_ptr = if layout.size() == 0 {
            ptr::null_mut()
        } else {
            let ptr = unsafe { self.get_raw_ptr_mut(entity_id.index) };
            if ptr.is_null() {
                panic!("Allocation failed during remove");
            }

            ptr
        };

        let src_ptr = unsafe { self.get_raw_ptr_mut(entity_id.index) };

        if self.layout.size() > 0 {
            unsafe { (self.write_fn)(src_ptr, raw_ptr) }
        }

        let resource = Resource::new(raw_ptr, None, None);

        Some(StoredResource::from_resource(
            resource, layout, self.drop_fn
        ))
    }

    pub fn insert(&mut self, entity_id: EntityId, stored_resource: StoredResource) -> Option<StoredResource> {
        // todo!("check entity id index can be written to");

        self.create_capacity(entity_id.index);

        if self.layout.size() > 0 {
            let dst = unsafe { self.get_raw_ptr_mut(entity_id.index) };
            unsafe { (self.write_fn)(stored_resource.ptr(), dst) };

            if stored_resource.layout().size() > 0 {
                unsafe { dealloc(stored_resource.ptr(), *stored_resource.layout()) };
            }
        }

        std::mem::forget(stored_resource);

        None
    }

    pub fn get_mut(&mut self, entity_id: &EntityId) -> Option<Resource> {
        if !self.entity_monitor.read().valid(entity_id) {
            return None
        }

        let ptr = unsafe { self.get_raw_ptr_mut(entity_id.index) };
        Some(Resource::new(ptr, None, None))
    }

    pub fn iter_mut(&mut self, target_entities: &[&EntityId]) -> Option<EntityUniqueView<'_>> {
        Some(EntityUniqueView::new(
            &self.pages,
            Arc::clone(&self.entity_monitor),
            target_entities.into_iter().map(|&target_entity_id| target_entity_id.clone()).collect(),
            self.layout.size(),
        ))
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