#[derive(PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Password {
    
}

impl From<u64> for Password {
    fn from(value: u64) -> Self {
        todo!()
    }
}