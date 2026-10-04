use aion_state::prelude::AccessorResult;

use crate::prelude::{TransmutableOwned, TransmutableShared};

pub mod transmutable;

pub enum AccessResult<T: TransmutableShared + TransmutableOwned> {
    Shared(T::AsShared),
    Unique(T),
    Owned(T::AsOwned)
}

impl<T: TransmutableShared + TransmutableOwned> AccessorResult<T> for AccessResult<T> {
    fn to_shared(value: T) -> Self {
        Self::Shared(<T as TransmutableShared>::transmute(value))
    }

    fn new_unique(value: T) -> Self {
        Self::Unique(value)
    }

    fn to_owned(value: T) -> Self {
        Self::Owned(<T as TransmutableOwned>::transmute(value))
    }
}
