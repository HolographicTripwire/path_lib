use crate::{obj_at_path::{ObjAtAppendablePath, ObjAtPath, OwnedObjAtPath}, paths::{PathPair, PathPrimitive}, HasChildren, HasDescendants, Path};

pub trait ObjAtPathWithChildren<'a,OldObj,Child,OldAtPath,Joiner>: ObjAtAppendablePath<'a, (), OldObj, Child, OldAtPath, Joiner> where
Child: 'a + PartialEq,
OldObj:'a + HasChildren<'a,Joiner,Child>,
OldAtPath:'a + Path,
Joiner:'a + PathPrimitive {
    fn valid_primitive_paths(&'a self) -> impl IntoIterator<Item = Joiner> { self._obj().valid_primitive_paths() }
    fn get_child(&'a self, path: &Joiner) -> Result<&'a Child,()> { self._obj().get_child(path) }
    fn get_located_child(&'a self, path: Joiner) -> Result<ObjAtPath<'a,Child,PathPair<OldAtPath,Joiner>>,()> {
        let child = self._obj().get_child(&path)?;
        let new_path = self._path().clone().pair_append(path);
        Ok(ObjAtPath::from_at(child, new_path))
    }
    
    fn get_children(&'a self) -> impl IntoIterator<Item = &'a Child> { 
        self.valid_primitive_paths()
            .into_iter()
            .map(|path| self.get_child(&path).expect("valid_primitive_paths returned an invalid path"))
    }
    fn get_located_children(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Child,PathPair<OldAtPath,Joiner>>> {
        self.valid_primitive_paths()
            .into_iter()
            .map(|path| self.get_located_child(path).expect("valid_primitive_paths returned an invalid path"))
    }
}
impl <'a,OldObj,Child,OldAtPath,Joiner,SelfType> ObjAtPathWithChildren<'a,OldObj,Child,OldAtPath,Joiner> for SelfType where
Child: 'a + PartialEq,
OldObj:'a + HasChildren<'a,Joiner,Child>,
OldAtPath:'a + Path,
Joiner:'a + PathPrimitive,
SelfType: ObjAtAppendablePath<'a, (), OldObj, Child, OldAtPath, Joiner> {}

pub trait ObjAtPathWithCloneChildren<'a, OldObj,Child,OldAtPath,Joiner>: ObjAtAppendablePath<'a, (), OldObj, Child, OldAtPath, Joiner> where
Child: 'a + PartialEq + Clone,
OldObj: 'a + HasChildren<'a,Joiner,Child>,
OldAtPath:Path,
Joiner:PathPrimitive {
    fn valid_primitive_paths(&'a self) -> impl IntoIterator<Item = Joiner> { self._obj().valid_primitive_paths() }
    fn get_child_owned(&self, path: &Joiner) -> Result<Child,()> { self._obj().get_child_owned(path) }
    fn get_located_child_owned(&self, path: Joiner) -> Result<OwnedObjAtPath<Child,PathPair<OldAtPath,Joiner>>,()> {
        let child = self._obj().get_child_owned(&path)?;
        let new_path = self._path().clone().pair_append(path);
        Ok(OwnedObjAtPath::from_at(child, new_path))
    }
    
    fn get_children_owned(&'a self) -> impl IntoIterator<Item = Child> { 
        self.valid_primitive_paths()
            .into_iter()
            .map(|path| self.get_child_owned(&path).expect("valid_primitive_paths returned an invalid path"))
    }
    fn get_located_children_owned(&'a self) -> impl IntoIterator<Item = OwnedObjAtPath<Child,PathPair<OldAtPath,Joiner>>> {
        self.valid_primitive_paths()
            .into_iter()
            .map(|path| self.get_located_child_owned(path).expect("valid_primitive_paths returned an invalid path"))
    }
}
impl <'a, OldObj,Child,OldAtPath,Joiner,SelfType> ObjAtPathWithCloneChildren<'a, OldObj,Child,OldAtPath,Joiner> for SelfType where
Child: 'a + PartialEq + Clone,
OldObj:'a + HasChildren<'a,Joiner,Child>,
OldAtPath:Path,
Joiner:PathPrimitive,
SelfType: ObjAtAppendablePath<'a, (), OldObj, Child, OldAtPath, Joiner> {}

pub trait ObjAtPathWithDescendants<'a,J,OldObj,Descendant,OldAtPath,Joiner>: ObjAtAppendablePath<'a, J, OldObj, Descendant, OldAtPath, Joiner> where
Descendant: 'a + PartialEq,
OldObj:'a + HasDescendants<'a,Joiner,J,Descendant>,
OldAtPath:'a + Path,
Joiner:'a + Path {
    fn valid_paths(&'a self) -> impl IntoIterator<Item = Joiner> { self._obj().valid_paths() }
    fn valid_paths_owned(&'a self) -> impl IntoIterator<Item = Joiner> where J: Clone, Descendant: Clone { self._obj().valid_paths_owned() }
    
    fn get_descendant(&'a self, path: &Joiner) -> Result<&'a Descendant,()> { self._obj().get_descendant(path) }
    fn get_descendant_owned(&self, path: &Joiner) -> Result<Descendant,()> where J: Clone, Descendant: Clone { self._obj().get_descendant_owned(path) }
    
    fn get_located_descendant(&'a self, path: Joiner) -> Result<ObjAtPath<'a,Descendant,PathPair<OldAtPath,Joiner>>,()> {
        let child = self._obj().get_descendant(&path)?;
        let new_path = self._path().clone().pair_append(path);
        Ok(ObjAtPath::from_at(child, new_path))
    }
    fn get_located_descendant_owned(&self, path: Joiner) -> Result<OwnedObjAtPath<Descendant,PathPair<OldAtPath,Joiner>>,()> where J: Clone, Descendant: Clone {
        let child = self._obj().get_descendant_owned(&path)?;
        let new_path = self._path().clone().pair_append(path);
        Ok(OwnedObjAtPath::from_at(child, new_path))
    }
    
    fn get_descendants(&'a self) -> impl IntoIterator<Item = &'a Descendant> { 
        self.valid_paths()
            .into_iter()
            .map(|path| self.get_descendant(&path).expect("valid_paths returned an invalid path"))
    }
    fn get_descendants_owned(&'a self) -> impl IntoIterator<Item = Descendant> where J: Clone, Descendant: Clone { 
        self.valid_paths_owned()
            .into_iter()
            .map(|path| self.get_descendant_owned(&path).expect("valid_paths returned an invalid path"))
    }

    fn get_located_descendants(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Descendant,PathPair<OldAtPath,Joiner>>> {
        self.valid_paths()
            .into_iter()
            .map(|path| self.get_located_descendant(path).expect("valid_paths returned an invalid path"))
    }
    fn get_located_descendants_owned(&'a self) -> impl IntoIterator<Item = OwnedObjAtPath<Descendant,PathPair<OldAtPath,Joiner>>> where J: Clone, Descendant: Clone {
        self.valid_paths_owned()
            .into_iter()
            .map(|path| self.get_located_descendant_owned(path).expect("valid_paths returned an invalid path"))
    }
}
impl <'a,J,OldObj,Descendant,OldAtPath,Joiner,SelfType> ObjAtPathWithDescendants<'a,J,OldObj,Descendant,OldAtPath,Joiner> for SelfType where
Descendant: 'a + PartialEq,
OldObj:'a + HasDescendants<'a,Joiner,J,Descendant>,
OldAtPath:'a + Path,
Joiner:'a + Path,
SelfType: ObjAtAppendablePath<'a, J, OldObj, Descendant, OldAtPath, Joiner> {}
