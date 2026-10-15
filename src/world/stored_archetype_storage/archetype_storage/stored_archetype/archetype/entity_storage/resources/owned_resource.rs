use crate::prelude::{TransmutableOwned, UniqueResource};

pub struct NotImplementedOwnedResource;

impl<'a> TransmutableOwned for UniqueResource {
    type AsOwned = NotImplementedOwnedResource;

    fn transmute(self) -> Self::AsOwned {
        unimplemented!()
    }
}