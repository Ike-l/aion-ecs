use crate::prelude::{Fetch, Optional, ResourceId, Transformer};

pub enum Or<L, R> { Left(L), Right(R), Both((L, R)) }

impl<OL: Optional, OR: Optional> Fetch for Or<OL, OR> {
    type Output<'a> = Or<OL, OR>;

    fn include_resource_ids() -> Vec<ResourceId> { vec![] }
}

impl<OL: Optional, OR: Optional> Optional for Or<OL, OR> {}
impl<TL: 'static, TR: 'static> Transformer for Or<TL, TR> {
    fn include_resource_ids() -> Vec<ResourceId> { vec![] }
} 