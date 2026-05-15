use hecs::Access;

pub enum TrackedAccess {
    Read(usize),
    Write,
    Iterate,
}

impl From<Access> for TrackedAccess {
    fn from(value: Access) -> Self {
        match value {
            Access::Iterate => Self::Iterate,
            Access::Read => Self::Read(1),
            Access::Write => Self::Write,
        }
    }
}