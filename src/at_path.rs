use std::marker::PhantomData;

use crate::{has_children::HasDescendants, paths::Path};

pub struct AtPath<'a,T,P,L,R,J> where
P: Path<L,R> {
    obj: &'a T,
    path: P,
    phantom: PhantomData<(L,R,J)>,
}
impl <'a,T,P,L,R,J> AtPath<'a,T,P,L,R,J> where
P: Path<L,R> {
    pub fn from_at(obj_at: &'a T, path: P) -> Self { Self { obj: obj_at, path, phantom: PhantomData }}
    pub fn from_in<O: HasDescendants<'a,L,R,P,J,T>>(obj_in: &'a O, path: P) -> Result<Self,()> {
        Ok(Self::from_at(obj_in.get_descendant(&path)?,path))
    }

    pub fn obj(&self) -> &'a T { &self.obj }
    pub fn path(&'a self) -> &'a P { &self.path } 
}
