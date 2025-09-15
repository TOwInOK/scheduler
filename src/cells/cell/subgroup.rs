use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]

pub enum SubGroup {
    A,
    B,
    All,
}

impl PartialEq for SubGroup {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (SubGroup::All, _)
                | (_, SubGroup::All)
                | (SubGroup::A, SubGroup::A)
                | (SubGroup::B, SubGroup::B)
        )
    }
}

impl Display for SubGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubGroup::A => write!(f, "А"),
            SubGroup::B => write!(f, "Б"),
            SubGroup::All => write!(f, "A&B"),
        }
    }
}

#[cfg(test)]
mod test_subgroup {
    use super::*;

    #[test]
    fn is_all_eq() {
        assert_eq!(SubGroup::A, SubGroup::All, "is A eq All");
        assert_eq!(SubGroup::B, SubGroup::All, "is B eq All");
        assert_eq!(SubGroup::All, SubGroup::All, "is All eq All");
    }
}
