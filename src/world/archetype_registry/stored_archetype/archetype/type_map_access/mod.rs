use aion_state::prelude::Accessor;

pub enum TypeMapAccess {
    
}

impl Accessor for TypeMapAccess {
    type StoredValue = ;

    type Value;

    type AccessResult<'a>;

    fn accepts_incoming(&self, incoming_access: &Self) -> bool {
        todo!()
    }

    fn can_insert_resource(&self) -> bool {
        todo!()
    }

    fn can_remove_resource(&self) -> bool {
        todo!()
    }

    fn acquire<'a>(
        &self, 
        stored_value: &'a mut Self::StoredValue
    ) -> Self::AccessResult<'a> {
        todo!()
    }

    fn merge(
        &mut self,
        incoming_access: Self
    ) {
        todo!()
    }

    fn release(
        &mut self,
        other: &Self
    ) {
        todo!()
    }

    fn insert(
        &self,
        value: Self::Value
    ) -> Self::StoredValue {
        todo!()
    }

    fn remove(
        &self,
        stored_value: Self::StoredValue
    ) -> Self::StoredValue {
        todo!()
    }
}