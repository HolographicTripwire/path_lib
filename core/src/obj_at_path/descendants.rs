use crate::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, paths::PathPair, HasDescendants, Path};


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
        let child = self.obj.get_descendant(&path)?;
        let new_path = self.path.clone().pair_append(path);
        Ok(ObjAtPath::from_inner(child, new_path))
    }
    pub fn get_located_descendants<'a,Joiner:Path,Descendant:'a,J>(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Descendant,PathPair<AtPath,Joiner>>> where Obj: HasDescendants<'a,Joiner,J,Descendant> {
        self.valid_paths()
            .into_iter()
            .map(|path| self.get_located_descendant(path).expect("valid_paths returned an invalid path"))
    }

    pub fn get_located_descendant_owned<'a,Joiner:Path,Descendant:Clone,J:Clone>(&'a self, path: Joiner) -> Result<OwnedObjAtPath<Descendant,PathPair<AtPath,Joiner>>,()> where Obj: HasDescendants<'a,Joiner,J,Descendant> {
        let child = self.obj.get_descendant_owned(&path)?;
        let new_path = self.path.clone().pair_append(path);
        Ok(OwnedObjAtPath::from_inner(child, new_path))
    }
    pub fn get_located_descendants_owned<'a,Joiner:Path,Descendant:Clone,J:Clone>(&'a self) -> impl IntoIterator<Item = OwnedObjAtPath<Descendant,PathPair<AtPath,Joiner>>> where Obj: HasDescendants<'a,Joiner,J,Descendant> {
        self.valid_paths()
            .into_iter()
            .map(|path| self.get_located_descendant_owned(path).expect("valid_paths returned an invalid path"))
    }
}

// pub trait ObjAtPathWithDescendants<'a,J,OldObj,Descendant,OldAtPath,Joiner> where
// Descendant: 'a + PartialEq,
// OldObj:'a + HasDescendants<'a,Joiner,J,Descendant>,
// OldAtPath: Path,
// Joiner: Path {
//     fn valid_paths(&'a self) -> impl IntoIterator<Item = Joiner> { self._obj().valid_paths() }
    
//     fn get_descendant(&'a self, path: &Joiner) -> Result<&'a Descendant,()> { self._obj().get_descendant(path) }
//     fn get_descendant_owned(&self, path: &Joiner) -> Result<Descendant,()> where J: Clone, Descendant: Clone { self._obj().get_descendant_owned(path) }
    
//     fn get_located_descendant(&'a self, path: Joiner) -> Result<ObjAtPath<'a,Descendant,PathPair<OldAtPath,Joiner>>,()> {
//         let child = self._obj().get_descendant(&path)?;
//         let new_path = self._path().clone().pair_append(path);
//         Ok(ObjAtPath::from_inner(child, new_path))
//     }
//     fn get_located_descendant_owned(&self, path: Joiner) -> Result<OwnedObjAtPath<Descendant,PathPair<OldAtPath,Joiner>>,()> where J: Clone, Descendant: Clone {
//         let child = self._obj().get_descendant_owned(&path)?;
//         let new_path = self._path().clone().pair_append(path);
//         Ok(OwnedObjAtPath::from_inner(child, new_path))
//     }
    
//     fn get_descendants(&'a self) -> impl IntoIterator<Item = &'a Descendant> { 
//         self.valid_paths()
//             .into_iter()
//             .map(|path| self.get_descendant(&path).expect("valid_paths returned an invalid path"))
//     }
//     fn get_descendants_owned(&'a self) -> impl IntoIterator<Item = Descendant> where J: Clone, Descendant: Clone { 
//         self.valid_paths()
//             .into_iter()
//             .map(|path| self.get_descendant_owned(&path).expect("valid_paths returned an invalid path"))
//     }

//     fn get_located_descendants(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Descendant,PathPair<OldAtPath,Joiner>>> {
//         self.valid_paths()
//             .into_iter()
//             .map(|path| self.get_located_descendant(path).expect("valid_paths returned an invalid path"))
//     }
//     fn get_located_descendants_owned(&'a self) -> impl IntoIterator<Item = OwnedObjAtPath<Descendant,PathPair<OldAtPath,Joiner>>> where J: Clone, Descendant: Clone {
//         self.valid_paths()
//             .into_iter()
//             .map(|path| self.get_located_descendant_owned(path).expect("valid_paths returned an invalid path"))
//     }
// }
// impl <'a,J,OldObj,Descendant,OldAtPath,Joiner,SelfType> ObjAtPathWithDescendants<'a,J,OldObj,Descendant,OldAtPath,Joiner> for SelfType where
// Descendant: 'a + PartialEq,
// OldObj:'a + HasDescendants<'a,Joiner,J,Descendant>,
// OldAtPath:'a + Path,
// Joiner:'a + Path {}
