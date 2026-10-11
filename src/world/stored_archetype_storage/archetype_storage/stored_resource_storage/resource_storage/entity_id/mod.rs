#[derive(Eq, PartialEq, Hash, Clone)]
pub struct EntityId {
    id: usize,
    epoch: usize
}