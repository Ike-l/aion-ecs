use crate::prelude::{TransmutableOwned, UniqueStoredResource};

pub struct NotImplementedOwnedResource;

impl<'a> TransmutableOwned for UniqueStoredResource {
    type AsOwned = NotImplementedOwnedResource;

    fn transmute(self) -> Self::AsOwned {
        unimplemented!()
    }
}