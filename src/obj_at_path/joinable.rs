use crate::{obj_at_path::ObjAtPath, paths::{AppendablePath, PrependablePath}, HasDescendants, Path};

pub trait ObjAtAppendablePath<'a,J,
OldObj: 'a + HasDescendants<'a,PathToAppend,J,NewObj>,
NewObj: 'a,
OldAtPath: 'a + AppendablePath<PathToAppend,Output=NewAtPath>,
PathToAppend: Path,
NewAtPath: 'a + Path> {
    fn _obj(&'a self) -> &'a OldObj;
    fn _path(&'a self) -> &'a OldAtPath;

    fn append(&'a self, subpath: PathToAppend) -> Result<ObjAtPath<'a,NewObj,NewAtPath>,()> {
        let obj = self._obj().get_descendant(&subpath)?;
        let path = self._path().clone().append(subpath);
        Ok(ObjAtPath::from_at(obj,path))
    }
}

impl <'a,J,
OldObj: 'a + HasDescendants<'a,PathToAppend,J,NewObj>,
NewObj: 'a,
OldAtPath: 'a + AppendablePath<PathToAppend,Output=NewAtPath>,
PathToAppend: Path,
NewAtPath: 'a + Path>
ObjAtAppendablePath<'a,J,OldObj,NewObj,OldAtPath,PathToAppend,NewAtPath> for
ObjAtPath<'a,OldObj,OldAtPath> {
    fn _obj(&'a self) -> &'a OldObj { self.obj() }
    fn _path(&'a self) -> &'a OldAtPath { self.path() }
}

pub trait ObjAtPrependablePath<'a,
Obj,
OldAtPath: 'a + PrependablePath<PathToPrepend,Output=NewAtPath>,
PathToPrepend: Path,
NewAtPath: 'a + Path> {
    fn _obj(&'a self) -> &'a Obj;
    fn _path(&'a self) -> &'a OldAtPath;

    fn prepend(&'a self, subpath: PathToPrepend) -> ObjAtPath<'a,Obj,NewAtPath> {
        let obj = self._obj();
        let path = self._path().clone().prepend(subpath);
        ObjAtPath::from_at(obj,path)
    }
}

impl <'a,
Obj,
OldAtPath: 'a + PrependablePath<PathToPrepend,Output=NewAtPath>,
PathToPrepend: Path,
NewAtPath: 'a + Path>
ObjAtPrependablePath<'a,Obj,OldAtPath,PathToPrepend,NewAtPath> for
ObjAtPath<'a,Obj,OldAtPath> {
    fn _obj(&'a self) -> &'a Obj { self.obj() }
    fn _path(&'a self) -> &'a OldAtPath { self.path() }
}