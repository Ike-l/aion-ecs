use std::{any::TypeId, marker::PhantomData};

use crate::prelude::ArchetypeId;

pub trait Query {
    fn request_archetype(archetype: &mut ArchetypeId);
    fn declare_yield(type_ids: &mut Vec<TypeId>);
}

impl<'a, T: 'static> Query for &'a T {
    fn request_archetype(archetype: &mut ArchetypeId) {
        archetype.insert(TypeId::of::<T>());
    }

    fn declare_yield(type_ids: &mut Vec<TypeId>) {
        type_ids.push(TypeId::of::<T>());
    }
}

struct With<Q> { _q: PhantomData<Q> }

impl<Q: Query> Query for With<Q> {
    fn request_archetype(archetype: &mut ArchetypeId) {
        Q::request_archetype(archetype);
    }

    fn declare_yield(_type_ids: &mut Vec<TypeId>) { }
}

struct Without<Q> { _q: PhantomData<Q> }

impl<Q: Query> Query for Without<Q> {
    fn request_archetype(archetype: &mut ArchetypeId) {
        let mut exclusion = ArchetypeId::default();
        Q::request_archetype(&mut exclusion);
        archetype.extract(&exclusion)
    }

    fn declare_yield(_type_ids: &mut Vec<TypeId>) { }
}

impl<A: Query, B: Query> Query for (A, B) {
    fn request_archetype(archetype: &mut ArchetypeId) {
        A::request_archetype(archetype);
        B::request_archetype(archetype);
    }

    fn declare_yield(type_ids: &mut Vec<TypeId>) {
        A::declare_yield(type_ids);
        B::declare_yield(type_ids);
    }
}
