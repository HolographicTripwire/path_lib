
use crate::{obj_at_path::ObjAtPath, paths::{Path, PathPrimitive}};

mod implementations;

const INVALID_PRIMITIVE_PATH_MESSAGE: &str = "valid_primitive_paths returned an invalid path";
const INVALID_PATH_MESSAGE: &str = "valid_paths returned an invalid path";

// Define HasChildren
pub trait HasChildren<'a, PathToChild, Child>: Sized where
PathToChild: PathPrimitive,
Child: 'a {
    fn valid_primitive_paths(&self) -> impl IntoIterator<Item = PathToChild>;
    fn get_child(&'a self, path: &PathToChild) -> Result<&'a Child,()>;
    
    fn get_located_child(&'a self, path: PathToChild) -> Result<ObjAtPath<'a,Child,PathToChild>,()>
        { Ok(ObjAtPath::from_at(self.get_child(&path)?,path)) }
    fn get_children(&'a self) -> impl IntoIterator<Item = &'a Child> {
        self.valid_primitive_paths()
            .into_iter()
            .map(|path| self.get_child(&path).expect(INVALID_PRIMITIVE_PATH_MESSAGE))
    }
    fn get_located_children(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Child,PathToChild>> {
        self.valid_primitive_paths()
            .into_iter()
            .map(|path| { self.get_located_child(path).expect(INVALID_PRIMITIVE_PATH_MESSAGE)})
    }
}

// Define HasDescendants
pub trait HasDescendants<'a,PathToDescendant,_Joiner,Descendant> where
Descendant: 'a, PathToDescendant:Path {
    fn valid_paths(&'a self) -> impl IntoIterator<Item=PathToDescendant>;
    fn get_descendant(&'a self, path: &PathToDescendant) -> Result<&'a Descendant,()>;

    fn get_located_descendant(&'a self, path: PathToDescendant) -> Result<ObjAtPath<'a,Descendant,PathToDescendant>,()>
        { Ok(ObjAtPath::from_at(self.get_descendant(&path)?, path)) }
    fn get_descendants(&'a self) -> impl IntoIterator<Item = &'a Descendant> {
        self.valid_paths()
            .into_iter()
            .map(|path| self.get_descendant(&path).expect(INVALID_PATH_MESSAGE))
    }
    fn get_located_descendants(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Descendant,PathToDescendant>> {
        self.valid_paths()
            .into_iter()
            .map(|path| { self.get_located_descendant(path).expect(INVALID_PATH_MESSAGE) })
    }
}
