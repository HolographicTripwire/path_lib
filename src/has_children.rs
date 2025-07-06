
use crate::{obj_at_path::ObjAtPath, paths::{Path, PathPair, PathPrimitive, PathSeries, PathUnit}};

// Define HasChildren
pub trait HasChildren<'a, Primitive, Child>: Sized where
Primitive: PathPrimitive,
Child: 'a {
    fn valid_primitive_paths(&self) -> impl IntoIterator<Item = Primitive>;
    fn get_child(&'a self, path: &Primitive) -> Result<&'a Child,()>;
    
    fn get_located_child(&'a self, path: Primitive) -> Result<ObjAtPath<'a,Child,Primitive>,()>
        { Ok(ObjAtPath::from_at(self.get_child(&path)?,path)) }
    fn get_children(&'a self) -> impl IntoIterator<Item = &'a Child> {
        self.valid_primitive_paths()
            .into_iter()
            .map(|path| self.get_child(&path).expect("valid_primitive_paths returned an invalid path"))
    }
    fn get_located_children(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Child,Primitive>> {
        self.valid_primitive_paths()
            .into_iter()
            .map(|path| { self.get_located_child(path).expect("valid_primitive_paths returned an invalid path")})
    }
}

// Define HasDescendants
pub trait HasDescendants<'a,PathToDescendant,_Joiner,Descendant> where
Descendant: 'a, PathToDescendant:Path {
    fn get_descendant(&'a self, path: &PathToDescendant) -> Result<&'a Descendant,()>;
    fn get_located_descendant(&'a self, path: PathToDescendant) -> Result<ObjAtPath<'a,Descendant,PathToDescendant>,()>
        { Ok(ObjAtPath::from_at(self.get_descendant(&path)?, path)) }
}

// Implement get_descendant for a Primitive path
impl <'a,Primitive,WithDescendants,Descendant> 
HasDescendants<'a,Primitive,(),Descendant>
for WithDescendants where
Descendant: 'a,
Primitive: PathPrimitive,
WithDescendants: HasChildren<'a,Primitive,Descendant> {
    fn get_descendant(&'a self, atom: &Primitive) -> Result<&'a Descendant,()>
        { self.get_child(atom) }
}

// Implement get_descendant for a switcher
impl <'a,WithDescendants,Joiner,Descendant> 
HasDescendants<'a,PathUnit,Joiner,Descendant>
for WithDescendants where
Descendant: 'a,
WithDescendants: HasChildren<'a,(),Descendant> {
    fn get_descendant(&'a self, _: &PathUnit) -> Result<&'a Descendant,()> {
        self.get_child(&())
    }
}

// Implement get_descendant for a series
impl <'a,Subpath,Type>
HasDescendants<'a,PathSeries<Subpath>,(),Type>
for Type where
Subpath: Path,
Type: 'a + HasDescendants<'a,Subpath,(),Type> {
    fn get_descendant(&'a self, path: &PathSeries<Subpath>) -> Result<&'a Type,()> {
        let mut result = self;
        for subpath in path.paths()
            { result = result.get_descendant(subpath)?; }
        Ok(result)
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
}
