
use crate::paths::{Path, PathPair, PathPrimitive, PathSeries, PathSwitcher};

// Define HasChildren
pub trait HasChildren<'a, AtomicPathType, Child>: 'a + Sized where
AtomicPathType: PathPrimitive,
Child: 'a {
    fn children(&'a self) -> impl IntoIterator<Item = &'a Child>;
    fn get_child(&'a self, path: &AtomicPathType) -> Result<&'a Child,()>;
}

// Define HasDescendants
pub trait HasDescendants<'a,DescendantPath,Joiner,Descendant> where
Descendant: 'a, DescendantPath:Path {
    fn get_descendant(&'a self, path: &DescendantPath) -> Result<&'a Descendant,()>;
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
HasDescendants<'a,PathSwitcher,Joiner,Descendant>
for WithDescendants where
Descendant: 'a,
WithDescendants: HasChildren<'a,(),Descendant> {
    fn get_descendant(&'a self, _: &PathSwitcher) -> Result<&'a Descendant,()> {
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
        let joiner = self.get_descendant(path.left())?;
        joiner.get_descendant(path.right())
    }
}
