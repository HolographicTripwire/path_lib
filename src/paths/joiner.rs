use std::marker::PhantomData;

use crate::paths::{Path, PathImpl};

pub struct PathJoiner<LL,L,RL,R>
(L,R,PhantomData<(LL,RL)>) where
L: Path<LL,()>, 
R: Path<RL,()>; 

impl <LL,L,RL,R> PathJoiner<LL,L,RL,R> where
L: Path<LL,()>, 
R: Path<RL,()> {
    pub fn new(left: L, right: R) -> Self { Self(left, right, PhantomData) }
    pub fn left(&self) -> &L { &self.0 }
    pub fn right(&self) -> &R { &self.1 }
}

impl <LL,L,RL,R> Into<PathImpl<LL,RL>> for PathJoiner<LL,L,RL,R> where
L: Path<LL,()>, 
R: Path<RL,()> { 
    fn into(self) -> PathImpl<LL,RL> { PathImpl::joiner(self.0.into(), self.1.into()) }
}

impl <LL,L,RL,R> Path<LL,RL> for PathJoiner<LL,L,RL,R> where
L: Path<LL,()>, 
R: Path<RL,()> {}
