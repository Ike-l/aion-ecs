use std::any::{TypeId, type_name};

pub struct Resource {
    pub(crate) ptr: *mut u8,
    pub(crate) id_metadata: Option<TypeId>,
    pub(crate) name_metadata: Option<String>
}

impl Resource {
    pub fn new(ptr: *mut u8, id_metadata: Option<TypeId>, name_metadata: Option<String>) -> Self {
        Self { ptr, id_metadata, name_metadata }
    }

    /// Safety:
    /// * Ensure `T` is compatible with `self`
    pub unsafe fn as_shared<T: 'static>(&self) -> &T {
        if let Some(type_id) = self.id_metadata {
            assert_eq!(
                TypeId::of::<T>(), 
                type_id, 
                "Type Mismatch! Attempted shared access of `Resource` as {}. But `self` is stored as {}", 
                type_name::<T>(), 
                self.name_metadata.as_deref().unwrap_or("a different type")
            );
        }
        
        unsafe { &*(self.ptr as *const T) }
    }

    /// Safety:
    /// * Ensure `T` is compatible with `self`
    pub unsafe fn as_unique<T: 'static>(&mut self) -> &mut T {
        if let Some(type_id) = self.id_metadata {
            assert_eq!(
                TypeId::of::<T>(), 
                type_id, 
                "Type Mismatch! Attempted unique access of `Resource` as {}. But `self` is stored as {}", 
                type_name::<T>(), 
                self.name_metadata.as_deref().unwrap_or("a different type")
            );
        }

        unsafe { &mut *(self.ptr as *mut T) }
    }

    /// Safety:
    /// * Ensure `T` is compatible with `self`
    pub unsafe fn copy_inner<T: Copy + 'static>(self) -> T {
        if let Some(type_id) = self.id_metadata {
            assert_eq!(
                TypeId::of::<T>(), 
                type_id, 
                "Type Mismatch! Attempted to copy `Resource` as {}. But `self` is stored as {}", 
                type_name::<T>(), 
                self.name_metadata.as_deref().unwrap_or("a different type")
            );
        }

        *unsafe { self.as_shared() }
    }
}

