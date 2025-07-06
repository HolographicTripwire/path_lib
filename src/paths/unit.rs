use crate::paths::{Path, PathSeries};

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
    use super::*;

    impl Into<PathSeries<PathUnit>> for PathUnit {
        fn into(self) -> PathSeries<PathUnit> { PathSeries::new([self]) }
    }
}
