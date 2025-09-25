use std::fmt::Display;

use crate::cells::cell::subgroup::SubGroup;

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum Groups {
    First(SubGroup),
    Second(SubGroup),
}
// Only for SQL
impl From<Groups> for i64 {
    fn from(val: Groups) -> Self {
        match val {
            Groups::First(sub_group) => match sub_group {
                SubGroup::A => 1,
                SubGroup::B => 2,
            },
            Groups::Second(sub_group) => match sub_group {
                SubGroup::A => 3,
                SubGroup::B => 4,
            },
        }
    }
}

impl From<i64> for Groups {
    fn from(val: i64) -> Self {
        match val {
            1 => Groups::First(SubGroup::A),
            2 => Groups::First(SubGroup::B),
            3 => Groups::Second(SubGroup::A),
            4 => Groups::Second(SubGroup::B),
            _ => panic!("Invalid value for Groups"),
        }
    }
}

impl From<Groups> for u8 {
    fn from(val: Groups) -> Self {
        match val {
            Groups::First(sub_group) => match sub_group {
                SubGroup::A => 1,
                SubGroup::B => 2,
            },
            Groups::Second(sub_group) => match sub_group {
                SubGroup::A => 3,
                SubGroup::B => 4,
            },
        }
    }
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
