use crate::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, paths::PathPair, HasCloneDescendants, HasDescendants, Path};

pub trait ObjAtAppendablePath<'a,J,
OldObj: 'a + HasDescendants<'a,PathToAppend,J,NewObj>,
NewObj: 'a + PartialEq,
OldAtPath: Path,
PathToAppend: Path> {
    fn _obj(&self) -> &OldObj;
    fn _path(&self) -> &OldAtPath;

    fn append(&'a self, subpath: PathToAppend) -> Result<ObjAtPath<'a,NewObj,PathPair<OldAtPath,PathToAppend>>,()> {
        let obj = self._obj().get_descendant(&subpath)?;
        let path = self._path().clone().pair_append(subpath);
        Ok(ObjAtPath::from_at(obj,path))
    }
}
pub trait CloneObjAtAppendablePath<J,
OldObj: HasCloneDescendants<PathToAppend,J,NewObj>,
NewObj: PartialEq + Clone,
OldAtPath: Path,
PathToAppend: Path> {
    fn _obj(&self) -> &OldObj;
    fn _path(&self) -> &OldAtPath;

    fn append(&self, subpath: PathToAppend) -> Result<OwnedObjAtPath<NewObj,PathPair<OldAtPath,PathToAppend>>,()> {
        let obj = self._obj().get_descendant_owned(&subpath)?;
        let path = self._path().clone().pair_append(subpath);
        Ok(OwnedObjAtPath::from_at(obj,path))
    }
}

impl <'a,J,
OldObj: 'a + PartialEq + HasDescendants<'a,PathToAppend,J,NewObj>,
NewObj: 'a + PartialEq,
OldAtPath: 'a + Path,
PathToAppend: Path>
ObjAtAppendablePath<'a,J,OldObj,NewObj,OldAtPath,PathToAppend> for
ObjAtPath<'a,OldObj,OldAtPath> {
    fn _obj(&self) -> &OldObj { self.obj() }
    fn _path(&self) -> &OldAtPath { self.path() }
}
