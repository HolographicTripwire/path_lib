mod inner;
mod atomic;
mod series;
mod joiner;

pub use atomic::*;
pub use series::*;
pub use joiner::*;


// Define PathImpl
pub enum PathImpl<'a,A,LA,LL,LR,L,RA,RL,RR,R> where
L: Path<LA,LL,LR>, LA: PathAtom<LL,LR,L>, LL: PathLeft<LA,LR,L>, LR: PathRight<LA,LL,L>,
R: Path<RA,RL,RR>, RA: PathAtom<RL,RR,R>, RL: PathLeft<RA,RR,R>, RR: PathRight<RA,RL,R> {
    Atomic(&'a AtomicPath<A>),
    Series(&'a PathSeries<LA,LL,LR,L>),
    Joiner(&'a PathJoiner<LA,LL,LR,L,RA,RL,RR,R>)
}

// Define Path
pub trait Path<A,L,R> {}

// Define PathAtom
trait PathAtom<L,R,P: Path<Self,L,R>>: Sized {}
impl <A:Sized,L,R,P:Path<A,L,R>> PathAtom<L,R,P> for A {}

// Define PathLeft
trait PathLeft<A,R,P: Path<A,Self,R>>: Sized {}
impl <A,L:Sized,R,P:Path<A,L,R>> PathLeft<A,R,P> for L {}

// Define PathRight
trait PathRight<A,L,P: Path<A,L,Self>>: Sized {}
impl <A,L,R:Sized,P:Path<A,L,R>> PathRight<A,L,P> for R {}
