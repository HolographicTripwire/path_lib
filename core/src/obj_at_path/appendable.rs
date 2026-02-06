use crate::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, paths::PathPair, HasDescendants, Path};

pub trait ObjAtAppendablePath<'a,J, OldObj: HasDescendants<'a,PathToAppend,J,NewObj>, NewObj, OldAtPath: Path, PathToAppend: Path> {
    fn _obj(&self) -> &OldObj;
    fn _path(&self) -> &OldAtPath;

    fn append(&'a self, subpath: PathToAppend) -> Result<ObjAtPath<'a,NewObj,PathPair<OldAtPath,PathToAppend>>,()> where OldObj: 'a {
        let obj = self._obj().get_descendant(&subpath)?;
        let path = self._path().clone().pair_append(subpath);
        Ok(ObjAtPath::from_inner(obj,path))
    }
    fn append_owned(&self, subpath: PathToAppend) -> Result<OwnedObjAtPath<NewObj,PathPair<OldAtPath,PathToAppend>>,()> where J: Clone, NewObj: Clone {
        let obj = self._obj().get_descendant_owned(&subpath)?;
        let path = self._path().clone().pair_append(subpath);
        Ok(OwnedObjAtPath::from_inner(obj,path))
    }
}

impl <
    'a,J,
    OldObj: 'a + PartialEq + HasDescendants<'a,PathToAppend,J,NewObj>,
    NewObj,
    OldAtPath: Path,
    PathToAppend: Path
> ObjAtAppendablePath<'a,J,OldObj,NewObj,OldAtPath,PathToAppend> for
ObjAtPath<'a,OldObj,OldAtPath> {
    fn _obj(&self) -> &OldObj { self.obj() }
    fn _path(&self) -> &OldAtPath { self.path() }
}

impl <
    'a, J,
    OldObj: 'a + Clone + PartialEq + HasDescendants<'a,PathToAppend,J,NewObj>,
    NewObj,
    OldAtPath: Path,
    PathToAppend: Path
> ObjAtAppendablePath<'a,J,OldObj,NewObj,OldAtPath,PathToAppend> for
OwnedObjAtPath<OldObj,OldAtPath> {
    fn _obj(&self) -> &OldObj { self.obj() }
    fn _path(&self) -> &OldAtPath { self.path() }
}
