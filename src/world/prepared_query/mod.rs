use std::{marker::PhantomData, sync::Arc};

use hecs::{Query, QueryBorrow};
use parking_lot::Mutex;

use crate::prelude::{ArchetypeTracker, World};

pub struct PreparedQuery<Q: Query> {
    tracker: Arc<Mutex<ArchetypeTracker>>,
    query_tracker: ArchetypeTracker,
    _q: PhantomData<Q>
}

impl<Q: Query> PreparedQuery<Q> {
    pub fn new(
        tracker: Arc<Mutex<ArchetypeTracker>>,
        query_tracker: ArchetypeTracker
    ) -> Self {
        Self { tracker, query_tracker, _q: PhantomData::default() }
    }

    pub fn query<'a>(&self, world: &'a World) -> QueryBorrow<'a, Q> {
        // Safety
        // We hold and drop accesses
        unsafe { world.query::<Q>() }
    }
}

impl<Q: Query> Drop for PreparedQuery<Q> {
    fn drop(&mut self) {
        self.tracker.lock().split(&mut self.query_tracker);
    }
}