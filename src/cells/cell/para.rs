use std::fmt::Display;

use serde::{Deserialize, Serialize};
use time::Time;
use time::macros::time;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub enum Para {
    First,
    Second,
    Third,
    Four,
    Five,
    Six,
    Seven,
}

impl Para {
    /// Return Start->End, Break direction
    pub fn time(&self) -> (Time, Time, Time) {
        match self {
            Para::First => (time!(9:00), time!(10:30), time!(0:10)),
            Para::Second => (time!(10:40), time!(12:10), time!(0:40)),
            Para::Third => (time!(12:50), time!(14:20), time!(0:10)),
            Para::Four => (time!(14:30), time!(16:00), time!(0:10)),
            Para::Five => (time!(16:10), time!(17:40), time!(0:05)),
            Para::Six => (time!(17:45), time!(19:15), time!(0:05)),
            Para::Seven => (time!(19:20), time!(20:50), time!(0:00)),
        }
    }
}

impl Display for Para {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Para::First => write!(f, "1"),
            Para::Second => write!(f, "2"),
            Para::Third => write!(f, "3"),
            Para::Four => write!(f, "4"),
            Para::Five => write!(f, "5"),
            Para::Six => write!(f, "6"),
            Para::Seven => write!(f, "7"),
        }
    }
}
