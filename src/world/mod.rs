use crate::prelude::{ArchetypeId, ArchetypeRegistry, Query};

pub mod archetype_registry;
pub mod query;

#[derive(Default)]
pub struct World {
    archetype_registry: ArchetypeRegistry,
}


impl World {
    // Query
    // impl Query on (), &T, &mut T, [(A,), (A, .. B) where A, .. B : Query], Option<T>, Entity, Or<L, R>, With<Q>, Without<Q>, Satisfies<Q>
    // Entity: yields (Entity, QueryYield)
    // Or: Holds either L, R or L,R
    // With: Skip entities not satisfying 
    // Without: Skip entities satisfying 
    // Satisfies: Iterable over all entities with bools for satisfying  

    // Query::? 

    // what types
    // filter entities
    // pub fn query<Q: Query>(&self) -> QueryBorrow<Q>

    pub fn query<Q: Query>(&self) {
        let mut archetype = ArchetypeId::default();
        Q::request_archetype(&mut archetype);

        let mut yields = Vec::default();
        Q::declare_yield(&mut yields);

        self.archetype_registry.find_at_least(&archetype);
    }
}

/*
resolve query: Find all archetypes with at least `types`
get the type maps for each `type`

some filter entities: for each type map, for each entity, yield type
none filter entities: for each type map, yield type


Query: 
result is iterator over either (Entity, QueryYield) or QueryYield
declares which type maps


*/