use std::{any::TypeId, collections::HashMap};

use crate::prelude::TypeAccess;

pub mod type_access;

#[derive(Default)]
pub struct TypeTracker {
    access_map: HashMap<TypeId, TypeAccess>
}

impl TypeTracker {
    fn try_access_shared(&mut self, type_id: TypeId, share_count: usize) -> bool {
        let entry = self.access_map.entry(type_id).or_default();
        match entry {
            TypeAccess::Shared(num) => {
                let Some(new_share_count) = num.checked_add(share_count) else { return false };
                *num = new_share_count;
                true
            },
            TypeAccess::Unique => false,
        }
    }

    fn try_access_unique(&mut self, type_id: TypeId) -> bool {
        let ok = self.access_map.get(&type_id).is_none_or(|current_access| matches!(current_access, TypeAccess::Shared(0)));
        
        if ok {
            self.access_map.insert(type_id, TypeAccess::Unique);
        }

        ok
    }

    pub fn try_access(&mut self, type_id: TypeId, access: TypeAccess) -> bool {
        match access {
            TypeAccess::Shared(access_count) => self.try_access_shared(type_id, access_count),
            TypeAccess::Unique => self.try_access_unique(type_id),
        }
    }

    fn deaccess_shared(&mut self, type_id: &TypeId, deaccess_count: usize) -> bool {
        let Some(current_access) = self.access_map.get_mut(&type_id) else { return true };
        match current_access {
            TypeAccess::Shared(current_share_count) => {
                let Some(new_share_count) = current_share_count.checked_sub(deaccess_count) else { return false };
                *current_share_count = new_share_count;
                true
            },
            TypeAccess::Unique => false,
        }
    }

    fn deaccess_unique(&mut self, type_id: &TypeId) -> bool {
        let Some(current_access) = self.access_map.get_mut(&type_id) else { return true };
        match current_access {
            TypeAccess::Shared(0) => true,
            TypeAccess::Shared(_) => false,
            current_access @ TypeAccess::Unique => {
                *current_access = TypeAccess::default();
                true
            },
        }
    }

    fn deaccess(&mut self, type_id: &TypeId, access: TypeAccess) -> bool {
        match access {
            TypeAccess::Shared(deaccess_count) => self.deaccess_shared(type_id, deaccess_count),
            TypeAccess::Unique => self.deaccess_unique(type_id),
        }
    }

    pub fn split(&mut self, mut other: Self) {
        for (type_id, access) in other.access_map.drain() {
            self.deaccess(&type_id, access);
        }
    }
}