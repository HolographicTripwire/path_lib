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

pub trait AtJoinablePath<'a,J,
OldTo: 'a + HasDescendants<'a,PathJoinerL,PathJoinerR,PathJoiner,J,NewTo>,
NewTo: 'a,
OldPathToL: Clone, OldPathToR: Clone, OldPathTo: 'a + JoinablePath<OldPathToL,OldPathToR,PathJoinerL,PathJoinerR,PathJoiner,OL=NewPathToL,OR=NewPathToR,Output=NewPathTo>,
PathJoinerL: Clone, PathJoinerR: Clone, PathJoiner: Path<PathJoinerL,PathJoinerR>,
NewPathToL: Clone, NewPathToR: Clone, NewPathTo: 'a + Path<NewPathToL,NewPathToR>> {
    fn _obj(&'a self) -> &'a OldTo;
    fn _path(&'a self) -> &'a OldPathTo;

    fn join(&'a self, subpath: PathJoiner) -> Result<AtPath<'a,NewTo,NewPathTo,NewPathToL,NewPathToR>,()> {
        let obj = self._obj().get_descendant(&subpath)?;
        let path = self._path().clone().join(subpath);
        Ok(AtPath::from_at(obj,path))
    }
}

impl <'a,J,
OldTo: 'a + HasDescendants<'a,PathJoinerL,PathJoinerR,PathJoiner,J,NewTo>,
NewTo: 'a,
OldPathToL: Clone, OldPathToR: Clone, OldPathTo: 'a + JoinablePath<OldPathToL,OldPathToR,PathJoinerL,PathJoinerR,PathJoiner,OL=NewPathToL,OR=NewPathToR,Output=NewPathTo>,
PathJoinerL: Clone, PathJoinerR: Clone, PathJoiner: Path<PathJoinerL,PathJoinerR>,
NewPathToL: Clone, NewPathToR: Clone, NewPathTo: 'a + Path<NewPathToL,NewPathToR>> 
AtJoinablePath<'a,J,OldTo,NewTo,OldPathToL,OldPathToR,OldPathTo,PathJoinerL,PathJoinerR,PathJoiner,NewPathToL,NewPathToR,NewPathTo> for
AtPath<'a,OldTo,OldPathTo,OldPathToL,OldPathToR> {
    fn _obj(&'a self) -> &'a OldTo { self.obj() }
    fn _path(&'a self) -> &'a OldPathTo { self.path() }
}
