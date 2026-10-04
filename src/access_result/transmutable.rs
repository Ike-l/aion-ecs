pub trait TransmutableShared {
    type AsShared;
    
    /// Called when an AccessResult creates a `Shared` access from a `Unique` access
    fn transmute(self) -> Self::AsShared;
    
}

pub trait TransmutableOwned {
    type AsOwned;

    /// Called when an AccessResult creates an `Owned` access from a `Unique` access
    fn transmute(self) -> Self::AsOwned;
}