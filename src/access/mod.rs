use aion_state::prelude::Accessor;

#[derive(PartialEq)]
pub enum Access {

}

impl Accessor for Access {
    fn accepts_incoming(&self, incoming_access: &Self) -> bool {
        todo!()
    }

    fn can_insert_resource(&self) -> bool {
        todo!()
    }

    fn can_remove_resource(&self) -> bool {
        todo!()
    }

    fn acquire<'a, V: aion_state::prelude::StoredValueTrait, R: aion_state::prelude::AccessorResult<'a, V::Value>>(
        &self, 
        stored_value: &'a mut V
    ) -> Option<R> {
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
}