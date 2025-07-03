use std::marker::PhantomData;

use crate::paths::{Path, PathImpl};

pub struct PathPair<LL,L,RL,R>
(L,R,PhantomData<(LL,RL)>) where
LL: Clone, L: Path<LL,()>, 
RL: Clone, R: Path<RL,()>; 

impl <LL,L,RL,R> PathPair<LL,L,RL,R> where
LL: Clone, L: Path<LL,()>, 
RL: Clone, R: Path<RL,()> {
    pub fn new(left: L, right: R) -> Self { Self(left, right, PhantomData) }
    pub fn left(&self) -> &L { &self.0 }
    pub fn right(&self) -> &R { &self.1 }
}

impl <LL,L,RL,R> From<(L,R)> for PathPair<LL,L,RL,R> where
LL: Clone, L: Path<LL,()>, 
RL: Clone, R: Path<RL,()> {
    fn from(value: (L,R)) -> Self { Self::new(value.0,value.1) }
}

impl <LL,L,RL,R> Into<PathImpl<LL,RL>> for PathPair<LL,L,RL,R> where
LL: Clone, L: Path<LL,()>, 
RL: Clone, R: Path<RL,()> { 
    fn into(self) -> PathImpl<LL,RL> { PathImpl::pair(self.0.into(), self.1.into()) }
}

impl <LL,L,RL,R> Path<LL,RL> for PathPair<LL,L,RL,R> where
LL: Clone, L: Path<LL,()>, 
RL: Clone, R: Path<RL,()> {}

impl <LL,L,RL,R> Clone for PathPair<LL,L,RL,R> where
LL: Clone, L: Path<LL,()>,
RL: Clone, R: Path<RL,()> {
    fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone(), PhantomData) }
}
