use std::marker::PhantomData;

use crate::{has_children::HasDescendants, paths::{JoinablePath, Path}};

pub struct AtPath<'a,To,PathTo,Left,Right> where
Left: Clone, Right: Clone, PathTo: Path<Left,Right> {
    obj: &'a To,
    path: PathTo,
    phantom: PhantomData<(Left,Right)>,
}
impl <'a,To,PathTo,Left,Right> AtPath<'a,To,PathTo,Left,Right> where
Left: Clone, Right: Clone, PathTo: Path<Left,Right> {
    pub fn from_at(obj_at: &'a To, path: PathTo) -> Self { Self { obj: obj_at, path, phantom: PhantomData }}
    pub fn from_in<Joiner,O: HasDescendants<'a,Left,Right,PathTo,Joiner,To>>(obj_in: &'a O, path: PathTo) -> Result<Self,()> {
        Ok(Self::from_at(obj_in.get_descendant(&path)?,path))
    }

    pub fn obj(&'a self) -> &'a To { &self.obj }
    pub fn path(&'a self) -> &'a PathTo { &self.path } 
}
