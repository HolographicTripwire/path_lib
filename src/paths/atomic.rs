use crate::paths::{inner::PathInner, Path, PathAtom, PathImpl, PathLeft, PathRight};

pub struct AtomicPath<Atom>(Atom);

impl <Atom> AtomicPath<Atom> {
    fn atom(&self) -> &Atom { &self.0 }
}

impl <A,L,R> Path<A,L,R> for AtomicPath<A> {}

impl <A,LA,LL,LR,L,RA,RL,RR,R> PathInner<A,LA,LL,LR,L,RA,RL,RR,R> for
AtomicPath<A> where
L: Path<LA,LL,LR>, LA: PathAtom<LL,LR,L>, LL: PathLeft<LA,LR,L>, LR: PathRight<LA,LL,L>,
R: Path<RA,RL,RR>, RA: PathAtom<RL,RR,R>, RL: PathLeft<RA,RR,R>, RR: PathRight<RA,RL,R> {
    fn as_impl<'a,>(&'a self) -> PathImpl<'a,A,LA,LL,LR,L,RA,RL,RR,R> {
        super::PathImpl::Atomic(self)
    }
}
