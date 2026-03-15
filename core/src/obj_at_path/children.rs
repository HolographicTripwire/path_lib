use crate::{HasChildren, Path, obj_at_path::{ObjAtPath, OwnedObjAtPath}, paths::{PathPair, PathPrimitive}};


impl <'a,Obj,AtPath: Path> ObjAtPath<'a,Obj,AtPath> {
    pub fn valid_primitive_paths<Joiner:PathPrimitive,Child>(&self) -> impl IntoIterator<Item = Joiner> where Obj: HasChildren<Joiner,Child>  
        { self.obj.valid_primitive_paths() }
    
    pub fn get_child<Joiner:PathPrimitive,Child>(&self, path: &Joiner) -> Result<&Child,()> where Obj: HasChildren<Joiner,Child>
        { self.obj.get_child(path) }
    pub fn get_children<Joiner:PathPrimitive,Child:'a>(&self) -> impl IntoIterator<Item = &Child> where Obj: HasChildren<Joiner,Child> { 
        self.valid_primitive_paths()
            .into_iter()
            .map(|path| self.get_child(&path).expect("valid_primitive_paths returned an invalid path"))
    }
    
    pub fn get_child_owned<Joiner:PathPrimitive,Child:Clone>(&self, path: &Joiner) -> Result<Child,()> where Obj: HasChildren<Joiner,Child>
        { self.obj.get_child_owned(path) }
    pub fn get_children_owned<Joiner:PathPrimitive,Child:Clone>(&self) -> impl IntoIterator<Item = Child> where Obj: HasChildren<Joiner,Child> { 
        self.valid_primitive_paths()
            .into_iter()
            .map(|path| self.get_child_owned(&path).expect("valid_primitive_paths returned an invalid path"))
    }

    pub fn get_located_child<Joiner:PathPrimitive,Child>(&'a self, path: Joiner) -> Result<ObjAtPath<'a,Child,PathPair<AtPath,Joiner>>,()> where Obj: HasChildren<Joiner,Child> {
        let child = self.obj.get_child(&path)?;
        let new_path = self.path.clone().pair_append(path);
        Ok(ObjAtPath::from_inner(child, new_path))
    }
    pub fn get_located_children<Joiner:PathPrimitive,Child:'a>(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Child,PathPair<AtPath,Joiner>>> where Obj: HasChildren<Joiner,Child> {
        self.valid_primitive_paths()
            .into_iter()
            .map(|path| self.get_located_child(path).expect("valid_primitive_paths returned an invalid path"))
    }

    pub fn get_located_child_owned<Joiner:PathPrimitive,Child:Clone>(&self, path: Joiner) -> Result<OwnedObjAtPath<Child,PathPair<AtPath,Joiner>>,()> where Obj: HasChildren<Joiner,Child> {
        let child = self.obj.get_child_owned(&path)?;
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


// pub trait ObjAtPathWithChildren<'a,OldObj,Child,OldAtPath,Joiner> where
// Child:  PartialEq,
// OldObj: HasChildren<Joiner,Child>,
// OldAtPath: Path,
// Joiner: PathPrimitive {
//     fn valid_primitive_paths(&'a self) -> impl IntoIterator<Item = Joiner> where OldObj: 'a { self._obj().valid_primitive_paths() }

//     fn get_child(&'a self, path: &Joiner) -> Result<&'a Child,()> where OldObj: 'a { self._obj().get_child(path) }
//     fn get_children(&'a self) -> impl IntoIterator<Item = &'a Child> where OldObj: 'a, Child: 'a { 
//         self.valid_primitive_paths()
//             .into_iter()
//             .map(|path| self.get_child(&path).expect("valid_primitive_paths returned an invalid path"))
//     }

//     fn get_child_owned(&self, path: &Joiner) -> Result<Child,()>  where Child: Clone
//         { self._obj().get_child_owned(path) }
//     fn get_children_owned(&'a self) -> impl IntoIterator<Item = Child> where OldObj: 'a, Child: Clone { 
//         self.valid_primitive_paths()
//             .into_iter()
//             .map(|path| self.get_child_owned(&path).expect("valid_primitive_paths returned an invalid path"))
//     }
    
//     fn get_located_child<'b>(&'b self, path: Joiner) -> Result<ObjAtPath<'b, Child,PathPair<OldAtPath,Joiner>>,()> where OldObj: 'b {
//         let child = self._obj().get_child(&path)?;
//         let new_path = self._path().clone().pair_append(path);
//         Ok(ObjAtPath::from_inner(child, new_path))
//     }
//     fn get_located_children(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Child,PathPair<OldAtPath,Joiner>>> where OldObj: 'a, Child: 'a {
//         self.valid_primitive_paths()
//             .into_iter()
//             .map(|path| self.get_located_child(path).expect("valid_primitive_paths returned an invalid path"))
//     }

//     fn get_located_child_owned(&self, path: Joiner) -> Result<OwnedObjAtPath<Child,PathPair<OldAtPath,Joiner>>,()> where Child: Clone {
//         let child = self._obj().get_child_owned(&path)?;
//         let new_path = self._path().clone().pair_append(path);
//         Ok(OwnedObjAtPath::from_inner(child, new_path))
//     }
//     fn get_located_children_owned(&'a self) -> impl IntoIterator<Item = OwnedObjAtPath<Child,PathPair<OldAtPath,Joiner>>> where OldObj: 'a, Child: Clone {
//         self.valid_primitive_paths()
//             .into_iter()
//             .map(|path| self.get_located_child_owned(path).expect("valid_primitive_paths returned an invalid path"))
//     }
//     fn into_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Child,PathPair<OldAtPath,Joiner>>> where OldObj: Clone, Child: Clone, Self: 'a;
// }
// impl <'a, OldObj,Child,OldAtPath,Joiner> ObjAtPathWithChildren<'a, OldObj,Child,OldAtPath,Joiner> for ObjAtPath<'a, OldObj, OldAtPath> where
// Child: PartialEq,
// OldObj: HasChildren<Joiner,Child>,
// OldAtPath: Path,
// Joiner: PathPrimitive {
//     fn into_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Child,PathPair<OldAtPath,Joiner>>> where OldObj: Clone, Child: Clone, Self: 'a {
//         self.obj().clone().into_located_children_owned().into_iter()
//             .map(move |obj| obj.prepend(self.path().to_owned()))
//     }
// }