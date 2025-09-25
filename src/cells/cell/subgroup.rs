use std::fmt::Display;

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum SubGroup {
    A,
    B,
}

impl Display for SubGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubGroup::A => write!(f, "А"),
            SubGroup::B => write!(f, "Б"),
        }
    }
}

// #[cfg(test)]
// mod test_subgroup {
//     use super::*;

//     #[test]
//     fn is_all_eq() {
//         assert_eq!(SubGroup::A, SubGroup::All, "is A eq All");
//         assert_eq!(SubGroup::B, SubGroup::All, "is B eq All");
//         assert_eq!(SubGroup::All, SubGroup::All, "is All eq All");
//     }
// }
