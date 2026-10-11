use std::{alloc::{Layout, alloc, dealloc}, any::{TypeId, type_name}, mem::forget, ptr::{self, NonNull}};

use crate::prelude::Resource;

pub mod resource;

pub struct StoredResource {
    resource: Resource,
    layout: Layout,
    drop_fn: unsafe fn(*mut u8),
}

impl StoredResource {
    pub fn new<T: 'static>(value: T) -> Self {
        let layout = Layout::new::<T>();
        let ptr = if layout.size() == 0 {
            ptr::null_mut()
        } else {
            let raw_ptr = unsafe { alloc(layout) };
            if raw_ptr.is_null() {
                panic!("Allocation failed");
            }
            unsafe {
                ptr::write(raw_ptr as *mut T, value);
            }
            raw_ptr
        };

        let resource = Resource::new(
            ptr, 
            Some(TypeId::of::<T>()), 
            Some(type_name::<T>().to_string()),
        );

        Self {
            resource,
            layout,
            drop_fn: |p| unsafe { ptr::drop_in_place(p as *mut T); },
        }
    }

    pub fn ptr(&self) -> *mut u8 {
        self.resource.ptr
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    /// # Safety
    /// * `T` must be compatible with `self`
    pub unsafe fn into_inner<T: 'static>(self) -> T {
        if let Some(type_id) = self.resource.id_metadata {
            assert_eq!(
                TypeId::of::<T>(), 
                type_id, 
                "Type Mismatch! Attempted to consume `StoredResource` as {}. But `self` is stored as {}", 
                type_name::<T>(), 
                self.resource.name_metadata.as_deref().unwrap_or("a different type")
            );
        }


        let value = if self.layout.size() == 0 {
            unsafe { ptr::read(NonNull::<T>::dangling().as_ptr()) }
        } else {
            unsafe { ptr::read(self.resource.ptr as *const T) }
        };

        if !self.resource.ptr.is_null() && self.layout.size() > 0 {
            unsafe { dealloc(self.resource.ptr, self.layout) };
        }

        forget(self);

        value
    }
}

impl Drop for StoredResource {
    fn drop(&mut self) {
        if !self.resource.ptr.is_null() && self.layout.size() > 0 {
            unsafe {
                (self.drop_fn)(self.resource.ptr);
                
                std::alloc::dealloc(self.resource.ptr, self.layout);
            }
        }
    }
}