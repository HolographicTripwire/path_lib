use crate::{HasDescendants, Path, obj_at_path::{ObjAtPath, OwnedObjAtPath}, paths::{PathPair}};

impl <'a,Obj,AtPath:Path> ObjAtPath<'a,Obj,AtPath> {
    pub fn append<PathToAppend: Path,J,NewObj>(&self, subpath: PathToAppend) -> Result<ObjAtPath<'a,NewObj,PathPair<AtPath,PathToAppend>>,()> where Obj: HasDescendants<'a,PathToAppend,J,NewObj> {
        let obj = self.obj.get_descendant(&subpath)?;
        let path = self.path.clone().pair_append(subpath);
        Ok(ObjAtPath::from_inner(obj,path))
    }

    pub fn append_owned<PathToAppend: Path,J:Clone,NewObj:Clone>(&self, subpath: PathToAppend) -> Result<OwnedObjAtPath<NewObj,PathPair<AtPath,PathToAppend>>,()> where Obj: HasDescendants<'a,PathToAppend,J,NewObj> {
        let obj = self.obj.get_descendant_owned(&subpath)?;
        let path = self.path.clone().pair_append(subpath);
        Ok(OwnedObjAtPath::from_inner(obj,path))
    }
}

impl <Obj,AtPath:Path> OwnedObjAtPath<Obj,AtPath> {
    pub fn append<'a,PathToAppend: Path,J,NewObj>(&'a self, subpath: PathToAppend) -> Result<ObjAtPath<'a,NewObj,PathPair<AtPath,PathToAppend>>,()> where Obj: HasDescendants<'a,PathToAppend,J,NewObj> {
        let obj = self.obj.get_descendant(&subpath)?;
        let path = self.path.clone().pair_append(subpath);
        Ok(ObjAtPath::from_inner(obj,path))
    }

    pub fn append_owned<'a,PathToAppend: Path,J:Clone,NewObj:Clone>(&self, subpath: PathToAppend) -> Result<OwnedObjAtPath<NewObj,PathPair<AtPath,PathToAppend>>,()> where Obj: HasDescendants<'a,PathToAppend,J,NewObj> {
        let obj = self.obj.get_descendant_owned(&subpath)?;
        let path = self.path.clone().pair_append(subpath);
        Ok(OwnedObjAtPath::from_inner(obj,path))
    }
}

// pub trait ObjAtOnceAppendablePath<OldObj: HasChildren<PathToAppend,NewObj>, NewObj: PartialEq, OldAtPath: Path, PathToAppend: PathPrimitive> {
//     fn _obj(&self) -> &OldObj;
//     fn _path(&self) -> &OldAtPath;

//     fn append_inner(&self, subpath: PathToAppend) -> Result<(&NewObj,PathPair<OldAtPath,PathToAppend>),()> {
//         let obj = self._obj().get_child(&subpath)?;
//         let path = self._path().clone().pair_append(subpath);
//         Ok((obj,path))
//     }
//     fn append<'b>(&'b self, subpath: PathToAppend) -> Result<ObjAtPath<'b,NewObj,PathPair<OldAtPath,PathToAppend>>,()> where OldObj: 'b {
//         let (obj,path) = self.append_inner(subpath)?;
//         Ok(ObjAtPath::from_inner(obj,path))
//     }
// }

// pub trait ObjAtAppendablePath<'a,J, OldObj: HasDescendants<'a,PathToAppend,J,NewObj>, NewObj, OldAtPath: Path, PathToAppend: Path> {
//     fn _obj(&self) -> &OldObj;
//     fn _path(&self) -> &OldAtPath;

//     fn append(&'a self, subpath: PathToAppend) -> Result<ObjAtPath<'a,NewObj,PathPair<OldAtPath,PathToAppend>>,()> where OldObj: 'a {
//         let obj = self._obj().get_descendant(&subpath)?;
//         let path = self._path().clone().pair_append(subpath);
//         Ok(ObjAtPath::from_inner(obj,path))
//     }
//     fn append_owned(&self, subpath: PathToAppend) -> Result<OwnedObjAtPath<NewObj,PathPair<OldAtPath,PathToAppend>>,()> where J: Clone, NewObj: Clone {
//         let obj = self._obj().get_descendant_owned(&subpath)?;
//         let path = self._path().clone().pair_append(subpath);
//         Ok(OwnedObjAtPath::from_inner(obj,path))
//     }
// }

// impl <
//     'a,J,
//     OldObj: 'a + PartialEq + HasDescendants<'a,PathToAppend,J,NewObj>,
//     NewObj,
//     OldAtPath: Path,
//     PathToAppend: Path
// > ObjAtAppendablePath<'a,J,OldObj,NewObj,OldAtPath,PathToAppend> for
// ObjAtPath<'a,OldObj,OldAtPath> {
//     fn _obj(&self) -> &OldObj { self.obj() }
//     fn _path(&self) -> &OldAtPath { self.path() }
// }

// impl <
//     'a, J,
//     OldObj: 'a + Clone + PartialEq + HasDescendants<'a,PathToAppend,J,NewObj>,
//     NewObj,
//     OldAtPath: Path,
//     PathToAppend: Path
// > ObjAtAppendablePath<'a,J,OldObj,NewObj,OldAtPath,PathToAppend> for
// OwnedObjAtPath<OldObj,OldAtPath> {
//     fn _obj(&self) -> &OldObj { self.obj() }
//     fn _path(&self) -> &OldAtPath { self.path() }
// }
