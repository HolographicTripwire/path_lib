use crate::paths::{Path, PathPrimitive};

#[derive(Clone)]
pub struct AtomicPath<Atom: PathPrimitive>(Atom);

impl <Atom: PathPrimitive> AtomicPath<Atom> {
    pub fn new(atom: Atom) -> Self { Self(atom) }
    pub fn atom(&self) -> &Atom { &self.0 }
}

impl <Atom: PathPrimitive> From<Atom> for AtomicPath<Atom> {
    fn from(value: Atom) -> Self { Self::new(value) }
}

impl <A: PathPrimitive> Path for AtomicPath<A> {}
