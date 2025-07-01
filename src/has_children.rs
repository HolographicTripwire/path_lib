
use crate::{paths::{AtomicPath, Path, PathJoiner, PathPrimitive, PathSeries, PathWrapper}, PathImpl};

// Define HasChildren
pub trait HasChildren<'a, AtomicPathType, Child>: 'a + Sized where
AtomicPathType: PathPrimitive,
Child: 'a {
    fn children(&'a self) -> impl IntoIterator<Item = &'a Child>;
    fn get_child(&'a self, path: &AtomicPathType) -> Result<&'a Child,()>;
}

// Define HasDescendants
pub trait HasDescendants<'a,L,R,P,Joiner,Descendant> where
Descendant: 'a,
P: Path<L,R> {
    fn get_descendant(&'a self, path: &P) -> Result<&'a Descendant,()>;
}

// Implement get_descendant for an atomic
impl <'a,AtomicPathType,WithDescendants,Descendant> 
HasDescendants<'a,AtomicPathType,(),AtomicPath<AtomicPathType>,(),Descendant>
for WithDescendants where
Descendant: 'a,
AtomicPathType: PathPrimitive,
WithDescendants: HasChildren<'a,AtomicPathType,Descendant> {
    fn get_descendant(&'a self, atom: &AtomicPath<AtomicPathType>) -> Result<&'a Descendant,()>
        { self.get_child(atom.atom()) }
}

// Implement get_descendant for a series
impl <'a,SubpathType,Subpath,WithDescendants,Descendant>
HasDescendants<'a,SubpathType,(),PathSeries<SubpathType,Subpath>,(),Descendant>
for WithDescendants where
Subpath: Path<SubpathType,()>,
Descendant: 'a + HasDescendants<'a,SubpathType,(),Subpath,(),Descendant>,
WithDescendants: HasDescendants<'a,SubpathType,(),Subpath,(),Descendant> {
    fn get_descendant(&'a self, path: &PathSeries<SubpathType,Subpath>) -> Result<&'a Descendant,()> {
        let (head, tail) = path.paths().split_first().unwrap();
        let mut result = self.get_descendant(head)?;
        for subpath in tail
            { result = result.get_descendant(subpath)?; }
        Ok(result)
    }
}

// Implement get_descendant for a joiner
impl <'a,LeftPathType,LeftPath,RightPathType,RightPath,WithDescendants,Joiner,Descendant> 
HasDescendants<'a,LeftPathType,RightPathType,PathJoiner<LeftPathType,LeftPath,RightPathType,RightPath>,Joiner,Descendant>
for WithDescendants where
LeftPath: Path<LeftPathType,()>,
RightPath: Path<RightPathType,()>,
Descendant: 'a,
Joiner: 'a + HasDescendants<'a,RightPathType,(),RightPath,(),Descendant>,
WithDescendants: HasDescendants<'a,LeftPathType,(),LeftPath,(),Joiner> {
    fn get_descendant(&'a self, path: &PathJoiner<LeftPathType,LeftPath,RightPathType,RightPath>) -> Result<&'a Descendant,()> {
        let joiner = self.get_descendant(path.left())?;
        joiner.get_descendant(path.right())
    }
}

// Implement get_descendant for a wrapper
impl <'a,LeftPathType,RightPathType,P,WithDescendants,Joiner,Descendant> 
HasDescendants<'a,PathImpl<LeftPathType,RightPathType>,(),PathWrapper<LeftPathType,RightPathType,P>,Joiner,Descendant>
for WithDescendants where
P: Path<LeftPathType,RightPathType>,
Descendant: 'a,
WithDescendants: HasDescendants<'a,LeftPathType,RightPathType,P,Joiner,Descendant> {
    fn get_descendant(&'a self, path: &PathWrapper<LeftPathType,RightPathType,P>) -> Result<&'a Descendant,()> {
        self.get_descendant(path.get_inner())
    }
}
