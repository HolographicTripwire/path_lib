use crate::{obj_at_path::{ObjAtAppendablePath, ObjAtPath}, paths::{AppendablePath, PathPrimitive}, HasChildren, Path};

pub trait ObjAtPathWithChildren<'a,OldObj,Child,OldAtPath,Joiner,NewAtPath,Output>: ObjAtAppendablePath<'a, (), OldObj, Child, OldAtPath, Joiner, NewAtPath> where
Child: 'a,
OldObj:'a + HasChildren<'a,Joiner,Child>,
OldAtPath:'a + AppendablePath<Joiner,Output=NewAtPath>,
Joiner:'a + PathPrimitive,
NewAtPath:'a + Path {
    fn valid_primitive_paths(&'a self) -> impl IntoIterator<Item = Joiner> { self._obj().valid_primitive_paths() }
    fn get_child(&'a self, path: &Joiner) -> Result<&'a Child,()> { self._obj().get_child(path) }
    fn get_located_child(&'a self, path: Joiner) -> Result<ObjAtPath<'a,Child,NewAtPath>,()> {
        let obj = self._obj();
        let child = obj.get_child(&path)?;
        let new_path = self._path().clone().append(path);
        Ok(ObjAtPath::from_at(child, new_path))
    }
    
    fn get_children(&'a self) -> impl IntoIterator<Item = &'a Child> { 
        self.valid_primitive_paths()
            .into_iter()
            .map(|path| self.get_child(&path).expect("valid_primitive_paths returned an invalid path"))
    }
    fn get_located_children(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Child,NewAtPath>> {
        self.valid_primitive_paths()
            .into_iter()
            .map(|path| self.get_located_child(path).expect("valid_primitive_paths returned an invalid path"))
    }
}
