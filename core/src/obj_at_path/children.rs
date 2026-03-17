use crate::{HasChildren, Path, obj_at_path::{ObjAtPath, OwnedObjAtPath}, paths::{PathPair, PathPrimitive}};

// For ObjAtPath
impl <'a,Obj,AtPath: Path> ObjAtPath<'a,Obj,AtPath> {
    pub fn valid_primitive_paths<Joiner:PathPrimitive,Child>(&self) -> impl IntoIterator<Item = Joiner> where Obj: HasChildren<Joiner,Child>  
        { self.obj.valid_primitive_paths() }
    
    pub fn get_child<Joiner:PathPrimitive,Child>(&self, path: &Joiner) -> Result<&Child,()> where Obj: HasChildren<Joiner,Child>
        { self.obj.get_child(path) }
    pub fn get_children<Joiner:PathPrimitive,Child:'a>(&self) -> impl IntoIterator<Item = &Child> where Obj: HasChildren<Joiner,Child>
        { self.obj.get_children() }
    
    pub fn get_child_owned<Joiner:PathPrimitive,Child:Clone>(&self, path: &Joiner) -> Result<Child,()> where Obj: HasChildren<Joiner,Child>
        { self.obj.get_child_owned(path) }
    pub fn get_children_owned<Joiner:PathPrimitive,Child:Clone>(&self) -> impl IntoIterator<Item = Child> where Obj: HasChildren<Joiner,Child>
        { self.obj.get_children_owned() }

    pub fn get_located_child<Joiner:PathPrimitive,Child>(&'a self, path: Joiner) -> Result<ObjAtPath<'a,Child,PathPair<AtPath,Joiner>>,()> where Obj: HasChildren<Joiner,Child> {
        let child = self.get_child(&path)?;
        let new_path = self.path.clone().pair_append(path);
        Ok(ObjAtPath::from_inner(child, new_path))
    }
    pub fn get_located_children<Joiner:PathPrimitive,Child:'a>(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Child,PathPair<AtPath,Joiner>>> where Obj: HasChildren<Joiner,Child> {
        self.valid_primitive_paths()
            .into_iter()
            .map(|path| self.get_located_child(path).expect("valid_primitive_paths returned an invalid path"))
    }

    pub fn get_located_child_owned<Joiner:PathPrimitive,Child:Clone>(&self, path: Joiner) -> Result<OwnedObjAtPath<Child,PathPair<AtPath,Joiner>>,()> where Obj: HasChildren<Joiner,Child> {
        let child = self.get_child_owned(&path)?;
        let new_path = self.path.clone().pair_append(path);
        Ok(OwnedObjAtPath::from_inner(child, new_path))
    }
    pub fn get_located_children_owned<Joiner:PathPrimitive,Child:Clone>(&'a self) -> impl IntoIterator<Item = OwnedObjAtPath<Child,PathPair<AtPath,Joiner>>> where Obj: HasChildren<Joiner,Child> {
        self.valid_primitive_paths()
            .into_iter()
            .map(|path| self.get_located_child_owned(path).expect("valid_primitive_paths returned an invalid path"))
    }
    pub fn into_located_children_owned<Joiner:PathPrimitive,Child:Clone>(self) -> impl IntoIterator<Item = OwnedObjAtPath<Child,PathPair<AtPath,Joiner>>> where Obj: HasChildren<Joiner,Child> + Clone, AtPath:'a {
        self.obj.clone().into_located_children_owned()
            .into_iter()
            .map(move |x| x.prepend(self.path.to_owned()))
    }
}

// For OwnedObjAtPath
impl <Obj,AtPath: Path> OwnedObjAtPath<Obj,AtPath> {
    pub fn valid_primitive_paths<Joiner:PathPrimitive,Child>(&self) -> impl IntoIterator<Item = Joiner> where Obj: HasChildren<Joiner,Child>  
        { self.obj.valid_primitive_paths() }
    
    pub fn get_child<Joiner:PathPrimitive,Child>(&self, path: &Joiner) -> Result<&Child,()> where Obj: HasChildren<Joiner,Child>
        { self.obj.get_child(path) }
    pub fn get_children<'a,Joiner:PathPrimitive,Child:'a>(&'a self) -> impl IntoIterator<Item = &'a Child> where Obj: HasChildren<Joiner,Child>
        { self.obj.get_children() }
    
    pub fn get_child_owned<Joiner:PathPrimitive,Child:Clone>(&self, path: &Joiner) -> Result<Child,()> where Obj: HasChildren<Joiner,Child>
        { self.obj.get_child_owned(path) }
    pub fn get_children_owned<Joiner:PathPrimitive,Child:Clone>(&self) -> impl IntoIterator<Item = Child> where Obj: HasChildren<Joiner,Child>
        { self.obj.get_children_owned() }

    pub fn get_located_child<'a,Joiner:PathPrimitive,Child>(&'a self, path: Joiner) -> Result<ObjAtPath<'a,Child,PathPair<AtPath,Joiner>>,()> where Obj: HasChildren<Joiner,Child> {
        let child = self.get_child(&path)?;
        let new_path = self.path.clone().pair_append(path);
        Ok(ObjAtPath::from_inner(child, new_path))
    }
    pub fn get_located_children<'a,Joiner:PathPrimitive,Child:'a>(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Child,PathPair<AtPath,Joiner>>> where Obj: HasChildren<Joiner,Child> {
        self.valid_primitive_paths()
            .into_iter()
            .map(|path| self.get_located_child(path).expect("valid_primitive_paths returned an invalid path"))
    }

    pub fn get_located_child_owned<Joiner:PathPrimitive,Child:Clone>(&self, path: Joiner) -> Result<OwnedObjAtPath<Child,PathPair<AtPath,Joiner>>,()> where Obj: HasChildren<Joiner,Child> {
        let child = self.get_child_owned(&path)?;
        let new_path = self.path.clone().pair_append(path);
        Ok(OwnedObjAtPath::from_inner(child, new_path))
    }
    pub fn get_located_children_owned<Joiner:PathPrimitive,Child:Clone>(&self) -> impl IntoIterator<Item = OwnedObjAtPath<Child,PathPair<AtPath,Joiner>>> where Obj: HasChildren<Joiner,Child> {
        self.valid_primitive_paths()
            .into_iter()
            .map(|path| self.get_located_child_owned(path).expect("valid_primitive_paths returned an invalid path"))
    }
    pub fn into_located_children_owned<Joiner:PathPrimitive,Child:Clone>(self) -> impl IntoIterator<Item = OwnedObjAtPath<Child,PathPair<AtPath,Joiner>>> where Obj: HasChildren<Joiner,Child> + Clone {
        self.obj.clone().into_located_children_owned()
            .into_iter()
            .map(move |x| x.prepend(self.path.to_owned()))
    }
}
