use crate::paths::{Path};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct PathSwitcher;

impl PathSwitcher {
    pub fn new() -> Self { Self }
}

impl Path for PathSwitcher {}

mod from {
    use super::*;

    impl From<()> for PathSwitcher {
        fn from(_: ()) -> Self { Self }
    }
}
mod into {}