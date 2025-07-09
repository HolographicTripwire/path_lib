use crate::{obj_at_path::ObjAtPath, paths::PathPair, HasDescendants, Path};

impl <'a,Obj: Clone, AtPath: Path> ObjAtPath<'a,Obj,AtPath> {
    fn into_owned(self) -> OwnedObjAtPath<Obj,AtPath> { OwnedObjAtPath::from_at(self.obj.clone(), self.path) }
}

#[derive(Clone)]
pub struct OwnedObjAtPath<Obj: Clone, AtPath:Path> {
    obj: Obj,
    path: AtPath,
}
impl <'a, Obj: 'a + Clone, AtPath:Path> OwnedObjAtPath<Obj,AtPath> {
    pub fn from_at(obj_at: Obj, path: AtPath) -> Self { Self { obj: obj_at, path }}
    pub fn from_in<Joiner,O: HasDescendants<'a, AtPath,Joiner,Obj>>(obj_in: &'a O, path: AtPath) -> Result<Self,()> {
        Ok(ObjAtPath::from_in(obj_in, path)?.into_owned())
    }

    pub fn obj(&'a self) -> &'a Obj { &self.obj }
    pub fn path(&'a self) -> &'a AtPath { &self.path } 
    
    pub fn prepend<PathToPrepend: Path>(&'a self, subpath: PathToPrepend) -> OwnedObjAtPath<Obj,PathPair<PathToPrepend,AtPath>> {
        let obj = self.obj();
        let path = self.path().clone().pair_prepend(subpath);
        OwnedObjAtPath::from_at(obj.clone(),path)
    }

    pub fn replace_path<NewPath: Path>(self, function: impl Fn(AtPath) -> NewPath) -> OwnedObjAtPath<Obj,NewPath> {
        OwnedObjAtPath::from_at(self.obj, (function)(self.path))
    }
}
