use crate::{has_descendants::INVALID_PATH_MESSAGE, paths::{PathPair, PathPrimitive, PathSeries}, HasChildren, HasCloneChildren, HasCloneDescendants, HasDescendants, Path};

// Implement get_descendant for a Primitive path
impl <'a,Primitive,WithDescendants,Descendant> 
HasDescendants<'a,Primitive,(),Descendant>
for WithDescendants where
Descendant: 'a + PartialEq,
Primitive: PathPrimitive,
WithDescendants: HasChildren<'a,Primitive,Descendant> {
    fn valid_paths(&self) -> impl IntoIterator<Item=Primitive>
        { self.valid_primitive_paths() }
    fn get_descendant(&'a self, atom: &Primitive) -> Result<&'a Descendant,()>
        { self.get_child(atom) }
}
// Implement get_owned_descendants for a Primitive path
impl <'a,Primitive,WithDescendants,Descendant> 
HasCloneDescendants<'a,Primitive,(),Descendant>
for WithDescendants where
Descendant: 'a + PartialEq + Clone,
Primitive: PathPrimitive,
WithDescendants: HasCloneChildren<'a,Primitive,Descendant> {
    fn valid_paths_owned(&self) -> impl IntoIterator<Item=Primitive>
        { self.valid_primitive_paths() }
    fn get_descendant_owned(&self, atom: &Primitive) -> Result<Descendant,()>
        { self.get_child_owned(atom) }
}

// Implement get_descendant for a series
impl <'a,Subpath,Type>
HasDescendants<'a,PathSeries<Subpath>,(),Type>
for Type where
Subpath: 'a + Path,
Type: 'a + PartialEq + HasDescendants<'a,Subpath,(),Type> {
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
                let valid_subpaths = <Self as HasDescendants<'a,PathSeries<Subpath>,(),Type>>::valid_paths(descendant);
                [PathSeries::new([path.clone()])].into_iter().chain(
                    valid_subpaths.into_iter().map(|mut subpath: PathSeries<Subpath>| { subpath.prepend(path.clone()); subpath }))
                    .collect::<Vec<_>>()
            })
    }
}
// Implement get_descendant_owned for a series
impl <'a,Subpath,Type>
HasCloneDescendants<'a,PathSeries<Subpath>,(),Type>
for Type where
Subpath: 'a + Path,
Type: 'a + PartialEq + Clone + HasCloneDescendants<'a,Subpath,(),Type> {
    fn get_descendant_owned(&self, path: &PathSeries<Subpath>) -> Result<Type,()> {
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

    fn valid_paths_owned(&self) -> impl IntoIterator<Item=PathSeries<Subpath>> {
        <Self as HasCloneDescendants<'a,Subpath,(),Type>>::valid_paths_owned(self).into_iter()
            .flat_map(move |path| {
                let descendant = self.get_descendant_owned(&path).expect(INVALID_PATH_MESSAGE);
                let valid_subpaths = <Self as HasCloneDescendants<'a,PathSeries<Subpath>,(),Type>>::valid_paths_owned(&descendant);
                [PathSeries::new([path.clone()])].into_iter().chain(
                    valid_subpaths.into_iter().map(|mut subpath: PathSeries<Subpath>| { subpath.prepend(path.clone()); subpath }))
                    .collect::<Vec<_>>()
            })
    }
}

// Implement get_descendant for a joiner
impl <'a,LeftPath,RightPath,WithDescendants,Joiner,Descendant> 
HasDescendants<'a,PathPair<LeftPath,RightPath>,Joiner,Descendant>
for WithDescendants where
LeftPath: Path, RightPath: Path,
Descendant: 'a + PartialEq,
Joiner: 'a + PartialEq + HasDescendants<'a,RightPath,(),Descendant>,
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

// Implement get_owned_descendant for a joiner
impl <'a,LeftPath,RightPath,WithDescendants,Joiner,Descendant> 
HasCloneDescendants<'a,PathPair<LeftPath,RightPath>,Joiner,Descendant>
for WithDescendants where
LeftPath: Path, RightPath: Path,
Descendant: 'a + PartialEq + Clone,
Joiner: 'a + PartialEq + HasCloneDescendants<'a,RightPath,(),Descendant> + Clone,
WithDescendants: HasCloneDescendants<'a,LeftPath,(),Joiner> {
    fn get_descendant_owned(&self, path: &PathPair<LeftPath,RightPath>) -> Result<Descendant,()> {
        let joiner = self.get_descendant_owned(&path.left)?;
        joiner.get_descendant_owned(&path.right)
    }
    
    fn valid_paths_owned(&self) -> impl IntoIterator<Item=PathPair<LeftPath,RightPath>> {
        <Self as HasCloneDescendants<'a,LeftPath,(),Joiner>>::valid_paths_owned(self).into_iter()
            .flat_map(move |path| {
                let descendant = self.get_descendant_owned(&path).expect(INVALID_PATH_MESSAGE); 
                let valid_paths = descendant.valid_paths_owned();
                valid_paths.into_iter().map(move |subpath| { subpath.pair_prepend(path.clone()) }).collect::<Vec<_>>()
            })
    }
}
