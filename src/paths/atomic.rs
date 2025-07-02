use crate::paths::{Path, PathImpl, PathPrimitive};

#[derive(Clone)]
pub struct AtomicPath<Atom: PathPrimitive>(Atom);

impl <Atom: PathPrimitive> AtomicPath<Atom> {
    pub fn new(atom: Atom) -> Self { Self(atom) }
    pub fn atom(&self) -> &Atom { &self.0 }
}

impl <A: PathPrimitive> Into<PathImpl<A,()>> for AtomicPath<A>
    { fn into(self) -> PathImpl<A,()> { PathImpl::atom(self.0) } }

impl <A: PathPrimitive> Path<A,()> for AtomicPath<A> {}
