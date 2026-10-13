use std::marker::PhantomData;

use crate::prelude::EntityId;

pub trait Queryable {
    type Output<'a> where Self: 'a;
}

struct QueryResult<T> { _p: PhantomData<T> }

impl Queryable for () {
    type Output<'a> = QueryResult<EntityId>;
}

impl<T: Fetch> Queryable for T {
    type Output<'a> = QueryResult<(EntityId, T::Output<'a>)> where Self: 'a;
}

impl<T: Filter> Queryable for ((), T) {
    type Output<'a> = QueryResult<EntityId> where Self: 'a;
}

impl<T1: Fetch, T2: Filter> Queryable for (T1, T2) {
    type Output<'a> = QueryResult<(EntityId, T1::Output<'a>)> where Self: 'a;
}

pub trait Fetch {
    type Output<'a> where Self: 'a;
}

impl<T: Data> Fetch for T {
    type Output<'a> = T::Output<'a> where Self: 'a;
}

impl<T: Data> Fetch for Option<T> {
    type Output<'a> = Option<T::Output<'a>> where Self: 'a;
}

impl<L, R> Fetch for And<L, R> where L: Fetch, R: Fetch {
    type Output<'a> = And<L::Output<'a>, R::Output<'a>> where Self: 'a;
}

impl<L, R> Fetch for Or<L, R> where L: Fetch, R: Fetch {
    type Output<'a> = Or<L::Output<'a>, R::Output<'a>> where Self: 'a;
}

// T1, T2, .., TN
impl<T1: Fetch, T2: Fetch> Fetch for (T1, T2) {
    type Output<'a> = (T1::Output<'a>, T2::Output<'a>) where Self: 'a;
}
impl<T1: Fetch, T2: Fetch, T3: Fetch> Fetch for (T1, T2, T3) {
    type Output<'a> = (T1::Output<'a>, T2::Output<'a>, T3::Output<'a>) where Self: 'a;
}
impl<T1: Fetch, T2: Fetch, T3: Fetch, T4: Fetch> Fetch for (T1, T2, T3, T4) {
    type Output<'a> = (T1::Output<'a>, T2::Output<'a>, T3::Output<'a>, T4::Output<'a>) where Self: 'a;
}

pub trait Filter {}
impl<T: Data> Filter for With<T> {}
impl<T: Data> Filter for Without<T> {}
// T1, T2, .., TN
impl<T1: Filter, T2: Filter> Filter for (T1, T2) {}
impl<T1: Filter, T2: Filter, T3: Filter> Filter for (T1, T2, T3) {}
impl<T1: Filter, T2: Filter, T3: Filter, T4: Filter> Filter for (T1, T2, T3, T4) {}

pub trait Data {
    type Output<'a> where Self: 'a;
}

impl<T> Data for &T {
    type Output<'a> = &'a T where Self: 'a;
}

impl<T> Data for &mut T {
    type Output<'a> = &'a mut T where Self: 'a;
}

pub struct And<L, R> {
    left: L,
    right: R
}
pub enum Or<L, R> {
    Left(L),
    Right(R),
    Both((L, R))
}
pub struct With<T> { _p: PhantomData<T> }
// With
//     Transforms query Q by skipping entities not satisfying query R
pub struct Without<T> { _p: PhantomData<T> }
// Without
//     Transforms query Q by skipping entities satisfying query R
