use std::any::TypeId;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ResourceId {
    type_id: TypeId    
}

impl ResourceId {
    pub fn from_type<T: 'static>() -> Self {
        Self { type_id: TypeId::of::<T>() }
    }
}