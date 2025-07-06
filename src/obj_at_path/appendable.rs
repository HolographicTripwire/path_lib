use crate::{obj_at_path::ObjAtPath, paths::PathPair, HasDescendants, Path};

pub trait ObjAtAppendablePath<'a,J,
OldObj: 'a + HasDescendants<'a,PathToAppend,J,NewObj>,
NewObj: 'a,
OldAtPath: 'a + Path,
PathToAppend: Path> {
    fn _obj(&'a self) -> &'a OldObj;
    fn _path(&'a self) -> &'a OldAtPath;

    fn append(&'a self, subpath: PathToAppend) -> Result<ObjAtPath<'a,NewObj,PathPair<OldAtPath,PathToAppend>>,()> {
        let obj = self._obj().get_descendant(&subpath)?;
        let path = self._path().clone().pair_append(subpath);
        Ok(ObjAtPath::from_at(obj,path))
    }
}

impl <'a,J,
OldObj: 'a + HasDescendants<'a,PathToAppend,J,NewObj>,
NewObj: 'a,
OldAtPath: 'a + Path,
PathToAppend: Path>
ObjAtAppendablePath<'a,J,OldObj,NewObj,OldAtPath,PathToAppend> for
ObjAtPath<'a,OldObj,OldAtPath> {
    fn _obj(&'a self) -> &'a OldObj { self.obj() }
    fn _path(&'a self) -> &'a OldAtPath { self.path() }
}
