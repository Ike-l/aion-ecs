use crate::prelude::ResourceId;

pub mod transforms;

pub trait Transformer {
    fn include_resource_ids() -> Vec<ResourceId>;
} 
