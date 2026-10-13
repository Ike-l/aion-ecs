use std::marker::PhantomData;

trait Queryable {}
impl Queryable for () {}
impl<T: Fetch> Queryable for T {}
impl<T: Filter> Queryable for ((), T) {}
impl<T1: Fetch, T2: Filter> Queryable for (T1, T2) {}

trait Fetch {}
impl<T> Fetch for &T {}
impl<T> Fetch for &mut T {}
impl<T: Data> Fetch for Option<T> {}
impl<L, R> Fetch for And<L, R> where L: Data, R: Data {}
impl<L, R> Fetch for Or<L, R> where L: Data, R: Data {}
// T1, T2, .., TN
impl<T1: Fetch, T2: Fetch, T3: Fetch, T4: Fetch> Fetch for (T1, T2, T3, T4) {}

trait Filter {}
impl<T: Data> Filter for With<T> {}
impl<T: Data> Filter for Without<T> {}
// T1, T2, .., TN
impl<T1: Filter, T2: Filter, T3: Filter, T4: Filter> Filter for (T1, T2, T3, T4) {}

trait Data: Fetch {}
impl<T> Data for &T {}
impl<T> Data for &mut T {}

struct And<L: Data, R: Data> {
    left: L,
    right: R
}
enum Or<L: Data, R: Data> {
    Left(L),
    Right(R),
    Both((L, R))
}
struct With<T: Data> { _p: PhantomData<T> }
struct Without<T: Data> { _p: PhantomData<T> }
