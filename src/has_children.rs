
use crate::{obj_at_path::ObjAtPath, paths::{Path, PathPair, PathPrimitive, PathSeries}};

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

// Implement get_descendant for a Primitive path
impl <'a,Primitive,WithDescendants,Descendant> 
HasDescendants<'a,Primitive,(),Descendant>
for WithDescendants where
Descendant: 'a,
Primitive: PathPrimitive,
WithDescendants: HasChildren<'a,Primitive,Descendant> {
    fn valid_paths(&self) -> impl IntoIterator<Item=Primitive>
        { self.valid_primitive_paths() }
    fn get_descendant(&'a self, atom: &Primitive) -> Result<&'a Descendant,()>
        { self.get_child(atom) }
}

// Implement get_descendant for a series
impl <'a,Subpath,Type>
HasDescendants<'a,PathSeries<Subpath>,(),Type>
for Type where
Subpath: 'a + Path,
Type: 'a + HasDescendants<'a,Subpath,(),Type> {
    fn get_descendant(&'a self, path: &PathSeries<Subpath>) -> Result<&'a Type,()> {
        let mut result = self;
        for subpath in path.paths()
            { result = result.get_descendant(subpath)?; }
        Ok(result)
    }
    
    fn valid_paths(&'a self) -> impl IntoIterator<Item=PathSeries<Subpath>> {
        <Self as HasDescendants<'a,Subpath,(),Type>>::valid_paths(self).into_iter()
            .flat_map(move |path| {
                let descendant = self.get_descendant(&path).expect(INVALID_PATH_MESSAGE);
                let valid_paths = <Self as HasDescendants<'a,PathSeries<Subpath>,(),Type>>::valid_paths(descendant);
                valid_paths.into_iter().map(|mut subpath: PathSeries<Subpath>| { subpath.prepend(path.clone()); subpath }).collect::<Vec<_>>()
            })
    }
}

// Implement get_descendant for a joiner
impl <'a,LeftPath,RightPath,WithDescendants,Joiner,Descendant> 
HasDescendants<'a,PathPair<LeftPath,RightPath>,Joiner,Descendant>
for WithDescendants where
LeftPath: Path, RightPath: Path,
Descendant: 'a,
Joiner: 'a + HasDescendants<'a,RightPath,(),Descendant>,
WithDescendants: HasDescendants<'a,LeftPath,(),Joiner> {
    fn get_descendant(&'a self, path: &PathPair<LeftPath,RightPath>) -> Result<&'a Descendant,()> {
        let joiner = self.get_descendant(&path.left)?;
        joiner.get_descendant(&path.right)
    }
    
    fn valid_paths(&'a self) -> impl IntoIterator<Item=PathPair<LeftPath,RightPath>> {
        <Self as HasDescendants<'a,LeftPath,(),Joiner>>::valid_paths(self).into_iter()
            .flat_map(move |path| {
                let descendant = self.get_descendant(&path).expect(INVALID_PATH_MESSAGE); 
                let valid_paths = descendant.valid_paths();
                valid_paths.into_iter().map(move |subpath| { subpath.pair_prepend(path.clone()) })
            })
    }
}
