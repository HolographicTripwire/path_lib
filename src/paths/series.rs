use std::marker::PhantomData;

use crate::paths::{inner::PathInner, Path, PathAtom, PathImpl, PathLeft, PathRight};

pub struct PathSeries<SA,SL,SR,S>(Vec<S>,PhantomData<(SA,SL,SR)>) where
S: Path<SA,SL,SR>;

impl <SA,SL,SR,S> PathSeries<SA,SL,SR,S> where
S: Path<SA,SL,SR> {
    fn paths(&self) -> &Vec<S> { &self.0 }
}

impl <A,LA,LL,LR,L,R> Path<A,L,R> for
PathSeries<LA,LL,LR,L> where
L: Path<LA,LL,LR>, LA: PathAtom<LL,LR,L>, LL: PathLeft<LA,LR,L>, LR: PathRight<LA,LL,L> {}

impl <A,LA,LL,LR,L,RA,RL,RR,R> PathInner<A,LA,LL,LR,L,RA,RL,RR,R> for
PathSeries<LA,LL,LR,L> where
L: Path<LA,LL,LR>, LA: PathAtom<LL,LR,L>, LL: PathLeft<LA,LR,L>, LR: PathRight<LA,LL,L>,
R: Path<RA,RL,RR>, RA: PathAtom<RL,RR,R>, RL: PathLeft<RA,RR,R>, RR: PathRight<RA,RL,R> {
    fn as_impl<'a,>(&'a self) -> PathImpl<'a,A,LA,LL,LR,L,RA,RL,RR,R> {
        super::PathImpl::Atomic(self)
    }
}
