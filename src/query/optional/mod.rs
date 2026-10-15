use crate::prelude::Fetch;

pub trait Optional: Fetch {}
impl<T: 'static> Optional for &T {}
impl<T: 'static> Optional for &mut T {}






impl<O1: Optional, O2: Optional> Optional for (O1, O2) {}
impl<O1: Optional, O2: Optional, O3: Optional> Optional for (O1, O2, O3) {}