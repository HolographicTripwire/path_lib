use std::marker::PhantomData;

use crate::paths::{Path, PathImpl};

pub struct PathWrapper<L,R,P>(P, PhantomData<(L,R)>) where P: Path<L,R>;

impl <L,R,P> PathWrapper<L,R,P> where P: Path<L,R> {
    pub fn new(to_wrap: P) -> Self { Self(to_wrap, PhantomData) }
    pub fn get_inner(&self) -> &P { &self.0 }
}

impl <L,R,P> Into<PathImpl<PathImpl<L,R>,()>> for PathWrapper<L,R,P> where P: Path<L,R>
    { fn into(self) -> PathImpl<PathImpl<L,R>,()> { PathImpl::wrapper(self.0.into()) } }

impl <L,R,P> Path<PathImpl<L,R>,()> for PathWrapper<L,R,P> where P: Path<L,R> {}
