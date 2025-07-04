use crate::paths::{Path};

#[derive(Clone,PartialEq,Eq)]
pub struct PathSwitcher;

impl PathSwitcher {
    pub fn new() -> Self { Self }
}

impl From<()> for PathSwitcher {
    fn from(_: ()) -> Self { Self }
}

impl Path for PathSwitcher {}
