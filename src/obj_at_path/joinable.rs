use crate::{obj_at_path::ObjAtPath, paths::JoinablePath, HasDescendants, Path};

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
