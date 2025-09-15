use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::subgroup::SubGroup;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]

pub enum Groups {
    First(SubGroup),
    Second(SubGroup),
    All,
}

impl PartialEq for Groups {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Groups::All, _) => true,
            (_, Groups::All) => true,
            (Groups::First(s1), Groups::First(s2)) => s1 == s2,
            (Groups::Second(s1), Groups::Second(s2)) => s1 == s2,
            _ => false,
        }
    }
}

impl Display for Groups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Groups::First(subgroup) => write!(f, "1{}", subgroup),
            Groups::Second(subgroup) => write!(f, "2{}", subgroup),
            Groups::All => write!(f, "Все группы"),
        }
    }
}

#[cfg(test)]
mod tests_groups {
    use super::*;

    #[test]
    fn is_eq_all() {
        assert_eq!(Groups::All, Groups::First(SubGroup::B));
        assert_eq!(Groups::All, Groups::First(SubGroup::A));
        assert_eq!(Groups::All, Groups::First(SubGroup::All));
        assert_eq!(Groups::All, Groups::Second(SubGroup::B));
        assert_eq!(Groups::All, Groups::Second(SubGroup::A));
        assert_eq!(Groups::All, Groups::Second(SubGroup::All));
        assert_eq!(Groups::First(SubGroup::All), Groups::First(SubGroup::All));
        assert_eq!(Groups::Second(SubGroup::All), Groups::Second(SubGroup::All));
    }

    #[test]
    fn is_not_eq_at_all() {
        assert_ne!(Groups::First(SubGroup::B), Groups::First(SubGroup::A));
        assert_ne!(Groups::Second(SubGroup::B), Groups::First(SubGroup::B));
        assert_ne!(Groups::Second(SubGroup::A), Groups::First(SubGroup::A));
        assert_ne!(Groups::Second(SubGroup::A), Groups::First(SubGroup::A));
        assert_ne!(Groups::Second(SubGroup::All), Groups::First(SubGroup::All));
    }
}
