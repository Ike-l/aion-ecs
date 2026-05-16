use std::{marker::PhantomData, sync::Arc};

use hecs::{Component, Entity, Ref};
use parking_lot::Mutex;

use crate::prelude::{ArchetypeTracker, World};

pub struct PrepareGetShared<T: Component> {
    tracker: Arc<Mutex<ArchetypeTracker>>,
    get_tracker: ArchetypeTracker,
    entity: Entity,
    _t: PhantomData<T>
}

impl<T: Component> PrepareGetShared<T> {
    pub fn new(
        tracker: Arc<Mutex<ArchetypeTracker>>,
        get_tracker: ArchetypeTracker,
        entity: Entity
    ) -> Self {
        Self { tracker, get_tracker, entity, _t: PhantomData::default() }
    }

    pub fn get<'a>(&self, world: &'a World) -> Ref<'a, T> {
        // Safety
        // We hold and drop accesses
        unsafe { world.get::<&T>(self.entity) }.expect("when accesses are tracked should ensure this does not fail")
    }
}

impl<T: Component> Drop for PrepareGetShared<T> {
    fn drop(&mut self) {
        self.tracker.lock().split(&mut self.get_tracker);
    }
}
