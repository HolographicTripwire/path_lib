use crate::paths::{Path, PathPrimitive};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct PathUnit;

impl PathUnit {
    pub fn new() -> Self { Self }
}

impl Path for PathUnit {}

mod from {
    use super::*;

    impl From<()> for PathUnit {
        fn from(_: ()) -> Self { Self }
    }
}
mod into {
}

#[derive(Clone)]
pub (crate) struct PrivatePathUnit;
impl PathPrimitive for PrivatePathUnit {}
impl <P: PathPrimitive> Path for P {}
