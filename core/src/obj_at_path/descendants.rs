use crate::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, paths::PathPair, HasDescendants, Path};

// For ObjAtPath
impl <'a,Obj,AtPath: Path> ObjAtPath<'a,Obj,AtPath> {
    pub fn valid_paths<Joiner:Path,Descendant,J>(&'a self) -> impl IntoIterator<Item = Joiner> where Obj: HasDescendants<'a,Joiner,J,Descendant>
        { self.obj.valid_paths() }
    
    pub fn get_descendant<Joiner:Path,Descendant,J>(&'a self, path: &Joiner) -> Result<&'a Descendant,()> where Obj: HasDescendants<'a,Joiner,J,Descendant>
        { self.obj.get_descendant(path) }
    pub fn get_descendants<Joiner:Path,Descendant:'a,J>(&'a self) -> impl IntoIterator<Item = &'a Descendant> where Obj: HasDescendants<'a,Joiner,J,Descendant> { 
        self.valid_paths()
            .into_iter()
            .map(|path| self.get_descendant(&path).expect("valid_paths returned an invalid path"))
    }

    pub fn get_descendant_owned<Joiner:Path,Descendant:Clone,J:Clone>(&'a self, path: &Joiner) -> Result<Descendant,()> where Obj: HasDescendants<'a,Joiner,J,Descendant>
        { self.obj.get_descendant_owned(path) }
    pub fn get_descendants_owned<Joiner:Path,Descendant:Clone,J:Clone>(&'a self) -> impl IntoIterator<Item = Descendant> where Obj: HasDescendants<'a,Joiner,J,Descendant> { 
        self.valid_paths()
            .into_iter()
            .map(|path| self.get_descendant_owned(&path).expect("valid_paths returned an invalid path"))
    }

    pub fn get_located_descendant<Joiner:Path,Descendant,J>(&'a self, path: Joiner) -> Result<ObjAtPath<'a,Descendant,PathPair<AtPath,Joiner>>,()> where Obj: HasDescendants<'a,Joiner,J,Descendant> {
        let descendant = self.get_descendant(&path)?;
        let new_path = self.path.clone().pair_append(path);
        Ok(ObjAtPath::from_inner(descendant, new_path))
    }
    pub fn get_located_descendants<Joiner:Path,Descendant:'a,J>(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Descendant,PathPair<AtPath,Joiner>>> where Obj: HasDescendants<'a,Joiner,J,Descendant> {
        self.valid_paths()
            .into_iter()
            .map(|path| self.get_located_descendant(path).expect("valid_paths returned an invalid path"))
    }

    pub fn get_located_descendant_owned<Joiner:Path,Descendant:Clone,J:Clone>(&'a self, path: Joiner) -> Result<OwnedObjAtPath<Descendant,PathPair<AtPath,Joiner>>,()> where Obj: HasDescendants<'a,Joiner,J,Descendant> {
        let child = self.get_descendant_owned(&path)?;
        let new_path = self.path.clone().pair_append(path);
        Ok(OwnedObjAtPath::from_inner(child, new_path))
    }
    pub fn get_located_descendants_owned<Joiner:Path,Descendant:Clone,J:Clone>(&'a self) -> impl IntoIterator<Item = OwnedObjAtPath<Descendant,PathPair<AtPath,Joiner>>> where Obj: HasDescendants<'a,Joiner,J,Descendant> {
        self.valid_paths()
            .into_iter()
            .map(|path| self.get_located_descendant_owned(path).expect("valid_paths returned an invalid path"))
    }
}


// For OwnedObjAtPath
impl <Obj,AtPath: Path> OwnedObjAtPath<Obj,AtPath> {
    pub fn valid_paths<'a,Joiner:Path,Descendant,J>(&'a self) -> impl IntoIterator<Item = Joiner> where Obj: HasDescendants<'a,Joiner,J,Descendant>
        { self.obj.valid_paths() }
    
    pub fn get_descendant<'a,Joiner:Path,Descendant,J>(&'a self, path: &Joiner) -> Result<&'a Descendant,()> where Obj: HasDescendants<'a,Joiner,J,Descendant>
        { self.obj.get_descendant(path) }
    pub fn get_descendants<'a,Joiner:Path,Descendant:'a,J>(&'a self) -> impl IntoIterator<Item = &'a Descendant> where Obj: HasDescendants<'a,Joiner,J,Descendant> { 
        self.valid_paths()
            .into_iter()
            .map(|path| self.get_descendant(&path).expect("valid_paths returned an invalid path"))
    }

    pub fn get_descendant_owned<'a,Joiner:Path,Descendant:Clone,J:Clone>(&'a self, path: &Joiner) -> Result<Descendant,()> where Obj: HasDescendants<'a,Joiner,J,Descendant>
        { self.obj.get_descendant_owned(path) }
    pub fn get_descendants_owned<'a,Joiner:Path,Descendant:Clone,J:Clone>(&'a self) -> impl IntoIterator<Item = Descendant> where Obj: HasDescendants<'a,Joiner,J,Descendant> { 
        self.valid_paths()
            .into_iter()
            .map(|path| self.get_descendant_owned(&path).expect("valid_paths returned an invalid path"))
    }

    pub fn get_located_descendant<'a,Joiner:Path,Descendant,J>(&'a self, path: Joiner) -> Result<ObjAtPath<'a,Descendant,PathPair<AtPath,Joiner>>,()> where Obj: HasDescendants<'a,Joiner,J,Descendant> {
        let descendant = self.get_descendant(&path)?;
        let new_path = self.path.clone().pair_append(path);
        Ok(ObjAtPath::from_inner(descendant, new_path))
    }
    pub fn get_located_descendants<'a,Joiner:Path,Descendant:'a,J>(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Descendant,PathPair<AtPath,Joiner>>> where Obj: HasDescendants<'a,Joiner,J,Descendant> {
        self.valid_paths()
            .into_iter()
            .map(|path| self.get_located_descendant(path).expect("valid_paths returned an invalid path"))
    }

    pub fn get_located_descendant_owned<'a,Joiner:Path,Descendant:Clone,J:Clone>(&'a self, path: Joiner) -> Result<OwnedObjAtPath<Descendant,PathPair<AtPath,Joiner>>,()> where Obj: HasDescendants<'a,Joiner,J,Descendant> {
        let child = self.get_descendant_owned(&path)?;
        let new_path = self.path.clone().pair_append(path);
        Ok(OwnedObjAtPath::from_inner(child, new_path))
    }
    pub fn get_located_descendants_owned<'a,Joiner:Path,Descendant:Clone,J:Clone>(&'a self) -> impl IntoIterator<Item = OwnedObjAtPath<Descendant,PathPair<AtPath,Joiner>>> where Obj: HasDescendants<'a,Joiner,J,Descendant> {
        self.valid_paths()
            .into_iter()
            .map(|path| self.get_located_descendant_owned(path).expect("valid_paths returned an invalid path"))
    }
}
