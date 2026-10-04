use aion_state::prelude::{Accessor, AccessorResult};

#[derive(PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Access {
    Shared(usize),
    Unique,
    Replace,
    UniqueToShared,
}

impl Accessor for Access {
    fn accepts_incoming(&self, incoming_access: &Self) -> bool {
        match (self, incoming_access) {
            (Access::UniqueToShared, _) => unreachable!("Access::UniqueToShared should never be stored"),
            (Access::Unique, Access::UniqueToShared) => true,
            (_, Access::UniqueToShared) => false,
            (Access::Unique, _) => false,
            (Access::Shared(0), _) => true,

            (Access::Shared(_), Access::Shared(_)) => true,
            (Access::Shared(_), Access::Unique) => false,
            (Access::Shared(_), Access::Replace) => false,

            (Access::Replace, _) => true,
        }
    }

    fn can_insert_resource(&self) -> bool {
        matches!(self, Self::Replace)
    }

    fn can_remove_resource(&self) -> bool {
        matches!(self, Self::Replace)
    }

    fn acquire<'a, V, R: AccessorResult<V>>(
        &self, 
        stored_value: V
    ) -> Option<R> {
        match self {
            Access::Shared(0) |
            Access::Replace |
            Access::UniqueToShared => None,
            Access::Shared(_) => Some(R::to_shared(stored_value)),
            Access::Unique => Some(R::new_unique(stored_value)),
        }
    }

    fn merge(
        &mut self,
        incoming_access: Self
    ) {
        match (self, incoming_access) {
            (current_access @ Access::Unique, Access::UniqueToShared) => *current_access = Access::Shared(1),
            (_, Access::UniqueToShared) => unreachable!(),
            (Access::UniqueToShared, _) => unreachable!(),

            (current_access @ Access::Shared(0), new_access @ _) => *current_access = new_access,
            (current_access @ Access::Replace, new_access @ _) => *current_access = new_access,
            
            (Access::Unique, Access::Shared(0)) => (),
            (Access::Unique, _) => unreachable!(),
            
            (Access::Shared(_), Access::Shared(0)) => (),
            (Access::Shared(n), Access::Shared(m)) => *n = n.saturating_add(m),
            (Access::Shared(_), Access::Unique) => unreachable!(),
            (Access::Shared(_), Access::Replace) => unreachable!(),
        }
    }

    fn release(
        &mut self,
        subtractor: &Self
    ) {
        match (self, subtractor) {
            (Access::Shared(_), Access::Shared(0)) => (),
            (Access::Shared(0), Access::Shared(_)) => unreachable!(),

            (Access::Shared(n), Access::Shared(m)) => {
                assert!(*n >= *m);
                
                *n = n.saturating_sub(*m)
            },
            (current_access @ Access::Unique, Access::Unique) => *current_access = Access::Shared(0),

            _  => unreachable!(),
        }
    }
}