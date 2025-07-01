use crate::paths::{Path, PathImpl};

pub struct PathSwitcher();

impl PathSwitcher {
    pub fn new() -> Self { Self() }
}

impl Into<PathImpl<(),()>> for PathSwitcher
    { fn into(self) -> PathImpl<(),()> { PathImpl::switcher() } }

    impl Path<(),()> for PathSwitcher {}
