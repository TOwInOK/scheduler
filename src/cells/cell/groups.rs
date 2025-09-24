use std::fmt::Display;

use crate::cells::cell::subgroup::SubGroup;

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum Groups {
    First(SubGroup),
    Second(SubGroup),
}

impl Display for Groups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Groups::First(subgroup) => write!(f, "1{}", subgroup),
            Groups::Second(subgroup) => write!(f, "2{}", subgroup),
        }
    }
}

#[cfg(test)]
mod tests_groups {
    use super::*;

    // #[test]
    // fn is_eq_all() {
    //     assert_eq!(Groups::First(SubGroup::All), Groups::First(SubGroup::All));
    //     assert_eq!(Groups::Second(SubGroup::All), Groups::Second(SubGroup::All));
    // }

    #[test]
    fn is_not_eq_at_all() {
        assert_ne!(Groups::First(SubGroup::B), Groups::First(SubGroup::A));
        assert_ne!(Groups::Second(SubGroup::B), Groups::First(SubGroup::B));
        assert_ne!(Groups::Second(SubGroup::A), Groups::First(SubGroup::A));
        assert_ne!(Groups::Second(SubGroup::A), Groups::First(SubGroup::A));
        // assert_ne!(Groups::Second(SubGroup::All), Groups::First(SubGroup::All));
    }
}
