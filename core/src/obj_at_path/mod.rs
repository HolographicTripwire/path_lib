mod appendable;
mod owned;
mod descendants;

use crate::{HasChildren, has_descendants::HasDescendants, paths::{Path, PathPair, PathPrimitive}};

pub use appendable::{ObjAtAppendablePath};
pub use owned::OwnedObjAtPath;
pub use descendants::{ObjAtPathWithChildren,ObjAtPathWithDescendants};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct ObjAtPath<'a, Obj, AtPath:Path> {
    obj: &'a Obj,
    path: AtPath,
}
impl <'a, Obj, AtPath:Path> ObjAtPath<'a,Obj,AtPath> {
    pub fn from_inner(obj_at: &'a Obj, path: AtPath) -> Self { Self { obj: obj_at, path }}
    pub fn from_outer<Joiner,O: HasDescendants<'a,AtPath,Joiner,Obj>>(obj_in: &'a O, path: AtPath) -> Result<Self,()> {
        Ok(Self::from_inner(obj_in.get_descendant(&path)?,path))
    }

    pub fn obj(&'a self) -> &'a Obj { &self.obj }
    pub fn path(&'a self) -> &'a AtPath { &self.path } 
    pub fn into_obj_and_path(self) -> (&'a Obj, AtPath) { (self.obj, self.path) }
    pub fn into_located_children<Child,PathToAppend>(self) -> impl IntoIterator<Item = ObjAtPath<'a,Child,PathPair<AtPath,PathToAppend>>> where Obj: HasChildren<PathToAppend,Child>, PathToAppend: PathPrimitive, Child: 'a {
        let (obj, old_path) = self.into_obj_and_path();
        obj.valid_primitive_paths().into_iter()
            .map(move |path| ObjAtPath::from_inner(obj.get_child(&path).expect("msg"), old_path.clone().pair_append(path)))
    }
    
    pub fn prepend<PathToPrepend: Path>(&'a self, subpath: PathToPrepend) -> ObjAtPath<'a,Obj,PathPair<PathToPrepend,AtPath>> {
        let obj = self.obj();
        let path = self.path().clone().pair_prepend(subpath);
        ObjAtPath::from_inner(obj,path)
    }

    pub fn replace_path<NewPath: Path>(self, function: impl Fn(AtPath) -> NewPath) -> ObjAtPath<'a,Obj,NewPath> {
        ObjAtPath::from_inner(self.obj, (function)(self.path))
    }
}
