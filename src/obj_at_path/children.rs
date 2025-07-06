use crate::{obj_at_path::{ObjAtAppendablePath, ObjAtPath}, paths::{PathPair, PathPrimitive}, HasChildren, Path};

pub trait ObjAtPathWithChildren<'a,OldObj,Child,OldAtPath,Joiner,NewAtPath,Output>: ObjAtAppendablePath<'a, (), OldObj, Child, OldAtPath, Joiner> where
Child: 'a,
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

impl <'a,OldObj,Child,OldAtPath,Joiner,NewAtPath,Output,SelfType> ObjAtPathWithChildren<'a,OldObj,Child,OldAtPath,Joiner,NewAtPath,Output> for SelfType where
Child: 'a,
OldObj:'a + HasChildren<'a,Joiner,Child>,
OldAtPath:'a + Path,
Joiner:'a + PathPrimitive,
SelfType: ObjAtAppendablePath<'a, (), OldObj, Child, OldAtPath, Joiner> {}
