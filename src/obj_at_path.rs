
use crate::{has_children::HasDescendants, paths::{JoinablePath, Path}};

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

pub trait ObjAtJoinablePath<'a,J,
OldObj: 'a + HasDescendants<'a,PathJoiner,J,NewObj>,
NewObj: 'a,
OldAtPath: 'a + JoinablePath<PathJoiner,Output=NewAtPath>,
PathJoiner: Path,
NewAtPath: 'a + Path> {
    fn _obj(&'a self) -> &'a OldObj;
    fn _path(&'a self) -> &'a OldAtPath;

    fn join(&'a self, subpath: PathJoiner) -> Result<ObjAtPath<'a,NewObj,NewAtPath>,()> {
        let obj = self._obj().get_descendant(&subpath)?;
        let path = self._path().clone().join(subpath);
        Ok(ObjAtPath::from_at(obj,path))
    }
}

impl <'a,J,
OldObj: 'a + HasDescendants<'a,PathJoiner,J,NewObj>,
NewObj: 'a,
OldAtPath: 'a + JoinablePath<PathJoiner,Output=NewAtPath>,
PathJoiner: Path,
NewAtPath: 'a + Path>
ObjAtJoinablePath<'a,J,OldObj,NewObj,OldAtPath,PathJoiner,NewAtPath> for
ObjAtPath<'a,OldObj,OldAtPath> {
    fn _obj(&'a self) -> &'a OldObj { self.obj() }
    fn _path(&'a self) -> &'a OldAtPath { self.path() }
}
