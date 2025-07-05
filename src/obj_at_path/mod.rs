mod joinable;

use crate::{has_children::HasDescendants, paths::{Path}};

pub use joinable::ObjAtAppendablePath;

pub struct ObjAtPath<'a, Obj, AtPath:Path> {
    obj: &'a Obj,
    path: AtPath,
}
impl <'a, Obj, AtPath:Path> ObjAtPath<'a,Obj,AtPath> {
    pub fn from_at(obj_at: &'a Obj, path: AtPath) -> Self { Self { obj: obj_at, path }}
    pub fn from_in<Joiner,O: HasDescendants<'a,AtPath,Joiner,Obj>>(obj_in: &'a O, path: AtPath) -> Result<Self,()> {
        Ok(Self::from_at(obj_in.get_descendant(&path)?,path))
    }

    pub fn obj(&'a self) -> &'a Obj { &self.obj }
    pub fn path(&'a self) -> &'a AtPath { &self.path } 
}
