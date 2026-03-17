use crate::{HasDescendants, Path, obj_at_path::{ObjAtPath, OwnedObjAtPath}, paths::{PathPair}};

// For ObjAtPath
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

// For OwnedObjAtPath
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
