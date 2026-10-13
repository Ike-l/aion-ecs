#[derive(Eq, PartialEq, Hash, Clone)]
pub struct EntityId {
    pub(crate) index: usize,
    pub(crate) epoch: usize
}