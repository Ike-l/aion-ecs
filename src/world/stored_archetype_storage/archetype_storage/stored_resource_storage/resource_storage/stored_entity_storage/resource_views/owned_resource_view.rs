use crate::prelude::{TransmutableOwned, UniqueResourceView};

pub struct NotImplementedOwnedResource;

impl<'a> TransmutableOwned for UniqueResourceView {
    type AsOwned = NotImplementedOwnedResource;

    fn transmute(self) -> Self::AsOwned {
        unimplemented!()
    }
}