/// [Path] objects, which objects can [have descendants](HasDescendants) at
pub mod paths;
/// Traits relating to objects which [have descendants](HasDescendants) at certain [Paths](Path).
mod has_descendants;
/// Implements [ObjAtPath](obj_at_path::ObjAtPath), for storing objects which are located at a known [Path] within another object
pub mod obj_at_path;

pub use has_descendants::{HasChildren,HasCloneChildren,HasDescendants,HasCloneDescendants};
pub use paths::{Path};

/// General implementations to use 
#[cfg(test)]
mod tests {
    use crate::paths::PathPrimitive;

    // Implement PathPrimitive for some primitive types to use in tests
    impl PathPrimitive for usize {}
    impl PathPrimitive for &str {}
}
