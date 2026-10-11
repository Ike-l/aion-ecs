#[derive(Eq, PartialEq, Hash, Clone)]
pub struct EntityId {
    pub(crate) id: usize,
    pub(crate) epoch: usize
}