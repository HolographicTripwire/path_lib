use crate::paths::{Path};

#[derive(Clone)]
pub struct PathSwitcher;

impl PathSwitcher {
    pub fn new() -> Self { Self }
}

impl From<()> for PathSwitcher {
    fn from(_: ()) -> Self { Self }
}

impl Path for PathSwitcher {}
