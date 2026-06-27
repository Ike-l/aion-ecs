#[derive(Clone)]
pub enum TypeAccess {
    Shared(usize),
    Unique,
}

impl Default for TypeAccess {
    fn default() -> Self {
        Self::Shared(0)
    }
}