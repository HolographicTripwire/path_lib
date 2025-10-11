mod appendable;
mod descendants;
mod owned;

use crate::{has_descendants::HasDescendants, paths::{Path, PathPair}};

pub use appendable::{ObjAtAppendablePath,CloneObjAtAppendablePath};
pub use descendants::{ObjAtPathWithChildren,ObjAtPathWithCloneChildren,ObjAtPathWithDescendants,ObjAtPathWithCloneDescendants};
pub use owned::OwnedObjAtPath;

#[derive(Clone)]
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
    
    pub fn prepend<PathToPrepend: Path>(&'a self, subpath: PathToPrepend) -> ObjAtPath<'a,Obj,PathPair<PathToPrepend,AtPath>> {
        let obj = self.obj();
        let path = self.path().clone().pair_prepend(subpath);
        ObjAtPath::from_at(obj,path)
    }

    pub fn replace_path<NewPath: Path>(self, function: impl Fn(AtPath) -> NewPath) -> ObjAtPath<'a,Obj,NewPath> {
        ObjAtPath::from_at(self.obj, (function)(self.path))
    }
}
