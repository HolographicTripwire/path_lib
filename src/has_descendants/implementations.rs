use crate::{has_descendants::INVALID_PATH_MESSAGE, paths::{PathPair, PathPrimitive, PathSeries}, HasChildren, HasDescendants, Path};

// Implement get_descendant for a Primitive path
mod primitive {
    use super::*;
    
    impl <'a,Primitive,WithDescendants,Descendant> 
    HasDescendants<'a,Primitive,(),Descendant>
    for WithDescendants where
    Descendant: PartialEq,
    Primitive: PathPrimitive,
    WithDescendants: HasChildren<Primitive,Descendant> {
        fn valid_paths(&self) -> Vec<Primitive>
            { self.valid_primitive_paths() }

        fn get_descendant(&self, atom: &Primitive) -> Result<&Descendant,()>
            { self.get_child(atom) }
        fn get_descendant_owned(&self, atom: &Primitive) -> Result<Descendant,()> where Descendant: Clone
            { self.get_child_owned(atom) }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::tests::TestTree1;

        #[test]
        fn test_valid_paths() {
            let tree = TestTree1::Tree(vec![TestTree1::Leaf(1),TestTree1::Tree(vec![TestTree1::Leaf(2)]),TestTree1::Leaf(3)]);
            let paths: Vec<usize> = <TestTree1 as HasDescendants<usize,_,TestTree1>>::valid_paths(&tree).into_iter().collect();
            assert_eq!(paths, vec![0,1,2])
        }
    }
}

// Implement get_descendant for a series
impl <'a,Subpath,SelfType>
HasDescendants<'a,PathSeries<Subpath>,(),SelfType>
for SelfType where
Subpath: Path,
SelfType: PartialEq + HasDescendants<'a,Subpath,(),SelfType> {
    fn get_descendant(&'a self, path: &PathSeries<Subpath>) -> Result<&'a SelfType,()> {
        let mut result = self;
        for subpath in path.paths()
            { result = result.get_descendant(subpath)?; }
        Ok(result)
    }
    fn get_descendant_owned(&self, path: &PathSeries<Subpath>) -> Result<SelfType,()> where SelfType: Clone {
        match path.paths().split_first() {
            // If there are subpath, iterate through the paths - getting each descendant
            Some((front,back)) => {
                let mut result = self.get_descendant_owned(front)?;
                for subpath in back
                    { result = result.get_descendant_owned(subpath)?; }
                Ok(result)
            },
            // Otherwise, just return itself
            None => Ok(self.clone()),
        }
    }
    
    fn valid_paths(&'a self) -> Vec<PathSeries<Subpath>> {
        <Self as HasDescendants<Subpath,(),SelfType>>::valid_paths(self).into_iter()
            .flat_map(move |path| {
                let descendant = self.get_descendant(&path).expect(INVALID_PATH_MESSAGE);
                let valid_subpaths = <Self as HasDescendants<PathSeries<Subpath>,(),SelfType>>::valid_paths(descendant);
                [PathSeries::new([path.clone()])].into_iter().chain(
                    valid_subpaths.into_iter().map(|mut subpath: PathSeries<Subpath>| { subpath.prepend(path.clone()); subpath }))
                    .collect::<Vec<_>>()
            }).collect()
    }
}

// Implement get_descendant for a joiner
impl <'a,LeftPath,RightPath,WithDescendants,Joiner,Descendant> 
HasDescendants<'a,PathPair<LeftPath,RightPath>,Joiner,Descendant>
for WithDescendants where
LeftPath: Path, RightPath: Path,
Descendant: PartialEq,
Joiner: 'a + PartialEq + HasDescendants<'a,RightPath,(),Descendant>,
WithDescendants: HasDescendants<'a,LeftPath,(),Joiner> {
    fn get_descendant(&'a self, path: &PathPair<LeftPath,RightPath>) -> Result<&'a Descendant,()> {
        let joiner = self.get_descendant(&path.left)?;
        joiner.get_descendant(&path.right)
    }
    fn get_descendant_owned(&self, path: &PathPair<LeftPath,RightPath>) -> Result<Descendant,()> where Joiner: Clone, Descendant: Clone {
        let joiner = self.get_descendant_owned(&path.left)?;
        joiner.get_descendant_owned(&path.right)
    }
    
    fn valid_paths(&'a self) -> Vec<PathPair<LeftPath,RightPath>> {
        <Self as HasDescendants<LeftPath,(),Joiner>>::valid_paths(self).into_iter()
            .flat_map(move |path| {
                let descendant = self.get_descendant(&path).expect(INVALID_PATH_MESSAGE); 
                let valid_paths = descendant.valid_paths();
                valid_paths.into_iter().map(move |subpath| { subpath.pair_prepend(path.clone()) })
            }).collect()
    }
}
