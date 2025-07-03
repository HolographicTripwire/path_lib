use std::marker::PhantomData;

use crate::paths::{Path, PathImpl};

pub struct PathSeries<L,P>(Vec<P>,PhantomData<L>) where
L: Clone, P:Path<L,()>;

impl <L,P> PathSeries<L,P> where 
L: Clone, P: Path<L,()> {
    pub fn new(series: Vec<P>) -> Self { Self(series, PhantomData) }
    pub fn paths(&self) -> &Vec<P> { &self.0 }
    pub fn into_paths(self) -> Vec<P> { self.0 }
}

impl <L,P> From<Vec<P>> for PathSeries<L,P> where
L:Clone, P: Path<L,()> {
    fn from(value: Vec<P>) -> Self { Self::new(value) }
}

impl <L,P> Into<PathImpl<L,()>> for PathSeries<L,P> where
L: Clone, P: Path<L,()>
    { fn into(self) -> PathImpl<L,()> { PathImpl::series(self.0.into_iter().map(|x| x.into()).collect()) } }

impl <L,P> Path<L,()> for PathSeries<L,P> where
L: Clone, P: Path<L,()> {}

impl <L,P> Clone for PathSeries<L,P> where
L: Clone, P: Path<L,()> {
    fn clone(&self) -> Self { Self(self.0.clone(), PhantomData) }
}
