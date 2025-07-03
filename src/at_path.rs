
use crate::{has_children::HasDescendants, paths::{JoinablePath, Path}};

pub struct AtPath<'a, To, PathTo:Path> {
    obj: &'a To,
    path: PathTo,
}
impl <'a, To, PathTo:Path> AtPath<'a,To,PathTo> {
    pub fn from_at(obj_at: &'a To, path: PathTo) -> Self { Self { obj: obj_at, path }}
    pub fn from_in<Joiner,O: HasDescendants<'a,PathTo,Joiner,To>>(obj_in: &'a O, path: PathTo) -> Result<Self,()> {
        Ok(Self::from_at(obj_in.get_descendant(&path)?,path))
    }

    pub fn obj(&'a self) -> &'a To { &self.obj }
    pub fn path(&'a self) -> &'a PathTo { &self.path } 
}

pub trait AtJoinablePath<'a,J,
OldTo: 'a + HasDescendants<'a,PathJoiner,J,NewTo>,
NewTo: 'a,
OldPathTo: 'a + JoinablePath<PathJoiner,Output=NewPathTo>,
PathJoiner: Path,
NewPathTo: 'a + Path> {
    fn _obj(&'a self) -> &'a OldTo;
    fn _path(&'a self) -> &'a OldPathTo;

    fn join(&'a self, subpath: PathJoiner) -> Result<AtPath<'a,NewTo,NewPathTo>,()> {
        let obj = self._obj().get_descendant(&subpath)?;
        let path = self._path().clone().join(subpath);
        Ok(AtPath::from_at(obj,path))
    }
}

impl <'a,J,
OldTo: 'a + HasDescendants<'a,PathJoiner,J,NewTo>,
NewTo: 'a,
OldPathTo: 'a + JoinablePath<PathJoiner,Output=NewPathTo>,
PathJoiner: Path,
NewPathTo: 'a + Path>
AtJoinablePath<'a,J,OldTo,NewTo,OldPathTo,PathJoiner,NewPathTo> for
AtPath<'a,OldTo,OldPathTo> {
    fn _obj(&'a self) -> &'a OldTo { self.obj() }
    fn _path(&'a self) -> &'a OldPathTo { self.path() }
}
