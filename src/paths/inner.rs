use std::marker::PhantomData;

use crate::{paths::{PathAtom, PathLeft, PathRight}, Path, PathImpl};

// Define PathInner
pub trait PathInner<A,LA,LL,LR,L,RA,RL,RR,R>: Path<A,L,R> where
L: Path<LA,LL,LR>, LA: PathAtom<LL,LR,L>, LL: PathLeft<LA,LR,L>, LR: PathRight<LA,LL,L>,
R: Path<RA,RL,RR>, RA: PathAtom<RL,RR,R>, RL: PathLeft<RA,RR,R>, RR: PathRight<RA,RL,R> {
    fn as_impl<'a>(&'a self) -> PathImpl<'a,A,LA,LL,LR,L,RA,RL,RR,R>;
}

// Define PathInnerStruct
struct PathInnerStruct<A,LA,LL,LR,L,RA,RL,RR,R,PI>(PI, PhantomData<(A,LA,LL,LR,L,RA,RL,RR,R)>);

// Implement Path for PathInnerStruct
impl <A,LA,LL,LR,L,RA,RL,RR,R,PI> Path<A,L,R> for PathInnerStruct<A,LA,LL,LR,L,RA,RL,RR,R,PI> where
L: Path<LA,LL,LR>, LA: PathAtom<LL,LR,L>, LL: PathLeft<LA,LR,L>, LR: PathRight<LA,LL,L>,
R: Path<RA,RL,RR>, RA: PathAtom<RL,RR,R>, RL: PathLeft<RA,RR,R>, RR: PathRight<RA,RL,R>,
PI: PathInner<A,LA,LL,LR,L,RA,RL,RR,R> {}

// Implement PathInner for PathInnerStruct
impl <A,LA,LL,LR,L,RA,RL,RR,R,PI> PathInner<A,LA,LL,LR,L,RA,RL,RR,R> for PathInnerStruct<A,LA,LL,LR,L,RA,RL,RR,R,PI> where
L: Path<LA,LL,LR>, LA: PathAtom<LL,LR,L>, LL: PathLeft<LA,LR,L>, LR: PathRight<LA,LL,L>,
R: Path<RA,RL,RR>, RA: PathAtom<RL,RR,R>, RL: PathLeft<RA,RR,R>, RR: PathRight<RA,RL,R>,
PI: PathInner<A,LA,LL,LR,L,RA,RL,RR,R> {
    fn as_impl<'a>(&'a self) -> PathImpl<'a,A,LA,LL,LR,L,RA,RL,RR,R> {
        todo!()
    }
}
