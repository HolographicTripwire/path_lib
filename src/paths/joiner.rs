use std::marker::PhantomData;

use crate::paths::{inner::PathInner, Path, PathAtom, PathImpl, PathLeft, PathRight};

pub struct PathJoiner<LA,LL,LR,L,RA,RL,RR,R>
(L, R, PhantomData<(LA,LL,LR,RA,RL,RR)>) where
L: Path<LA,LL,LR>, LA: PathAtom<LL,LR,L>, LL: PathLeft<LA,LR,L>, LR: PathRight<LA,LL,L>,
R: Path<RA,RL,RR>, RA: PathAtom<RL,RR,R>, RL: PathLeft<RA,RR,R>, RR: PathRight<RA,RL,R>;

impl <LA,LL,LR,L,RA,RL,RR,R>
PathJoiner<LA,LL,LR,L,RA,RL,RR,R> where
L: Path<LA,LL,LR>, LA: PathAtom<LL,LR,L>, LL: PathLeft<LA,LR,L>, LR: PathRight<LA,LL,L>,
R: Path<RA,RL,RR>, RA: PathAtom<RL,RR,R>, RL: PathLeft<RA,RR,R>, RR: PathRight<RA,RL,R> {
    pub fn left(&self) -> &L { &self.0 }
    pub fn right(&self) -> &R { &self.1 }
}

impl <A,LA,LL,LR,L,RA,RL,RR,R> Path<A,L,R> for
PathJoiner<LA,LL,LR,L,RA,RL,RR,R> where
L: Path<LA,LL,LR>, LA: PathAtom<LL,LR,L>, LL: PathLeft<LA,LR,L>, LR: PathRight<LA,LL,L>,
R: Path<RA,RL,RR>, RA: PathAtom<RL,RR,R>, RL: PathLeft<RA,RR,R>, RR: PathRight<RA,RL,R> {}

impl <A,LA,LL,LR,L,RA,RL,RR,R> PathInner<A,LA,LL,LR,L,RA,RL,RR,R> for
PathJoiner<LA,LL,LR,L,RA,RL,RR,R> where
L: Path<LA,LL,LR>, LA: PathAtom<LL,LR,L>, LL: PathLeft<LA,LR,L>, LR: PathRight<LA,LL,L>,
R: Path<RA,RL,RR>, RA: PathAtom<RL,RR,R>, RL: PathLeft<RA,RR,R>, RR: PathRight<RA,RL,R> {
    fn as_impl<'a,>(&'a self) -> PathImpl<'a,A,LA,LL,LR,L,RA,RL,RR,R> {
        super::PathImpl::Joiner(self)
    }
}
