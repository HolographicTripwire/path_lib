/// [Path] objects, which objects can [have descendants](HasDescendants) at
pub mod paths;
/// Traits relating to objects which [have descendants](HasDescendants) at certain [Paths](Path).
mod has_descendants;
/// Implements [ObjAtPath](obj_at_path::ObjAtPath), for storing objects which are located at a known [Path] within another object
pub mod obj_at_path;

pub use has_descendants::{HasChildren,HasDescendants};
pub use paths::{Path};

/// General implementations to use 
#[cfg(test)]
mod tests {
    use crate::{HasChildren, obj_at_path::{OwnedObjAtPath}, paths::PathPrimitive};

    // Implement PathPrimitive for some primitive types to use in tests
    impl PathPrimitive for usize {}
    impl PathPrimitive for &str {}

    #[derive(PartialEq,Eq)]
    pub enum TestTree1 {
        Leaf(u8),
        Tree(Vec<TestTree1>)
    }

    impl HasChildren<usize, TestTree1> for TestTree1 {
        fn valid_primitive_paths(&self) -> Vec<usize> {
            match self {
                TestTree1::Leaf(_) => 0..0,
                TestTree1::Tree(test_trees) => 0..test_trees.len(),
            }.collect()
        }

        fn get_child_owned(&self, _: &usize) -> Result<TestTree1,()> { unimplemented!() }
        #[allow(unreachable_code)]
        fn to_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<TestTree1, usize>> { unimplemented!(); vec![] }
    
        fn get_child(&self, path: &usize) -> Result<&TestTree1,()> {
            return match self {
                TestTree1::Leaf(_) => Err(()),
                TestTree1::Tree(test_trees) => Ok(&test_trees[*path]),
            };
        }
    }

    #[derive(PartialEq,Eq,Clone)]
    pub enum TestTree2 {
        Leaf(u8),
        Tree(Vec<TestTree2>)
    }

    impl HasChildren<usize, TestTree2> for TestTree2 {
        fn valid_primitive_paths(&self) -> Vec<usize> { match self {
            TestTree2::Leaf(_) => 0..0,
            TestTree2::Tree(test_trees) => 0..test_trees.len(),
        }.collect()}
    
        fn get_child(&self, path: &usize) -> Result<&TestTree2,()> { match self {
            TestTree2::Leaf(_) => Err(()),
            TestTree2::Tree(test_trees) => Ok(&test_trees[*path]),
        }}

        fn to_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<TestTree2,usize>> where TestTree2: Clone {
            match self {
                TestTree2::Leaf(_) => vec![],
                TestTree2::Tree(subtree) => subtree.into_iter()
                    .enumerate()
                    .map(|x| OwnedObjAtPath::from_inner(x.1, x.0))
                    .collect(),
            }
        }
        
        fn get_child_owned(&self, path: &usize) -> Result<TestTree2,()> where TestTree2: Clone { match self {
            TestTree2::Leaf(_) => Err(()),
            TestTree2::Tree(test_trees) => Ok(test_trees[*path].clone()),
        }}
    }
}
