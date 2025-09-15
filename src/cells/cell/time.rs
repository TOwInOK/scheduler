use serde::{Deserialize, Serialize};
use time::UtcDateTime;

use super::day::Day;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]

pub enum TimeCellRepiter {
    Regular(Day),
    Once(Day, UtcDateTime),
}
impl From<time::Weekday> for Day {
    fn from(value: time::Weekday) -> Self {
        match value {
            time::Weekday::Monday => Day::First,
            time::Weekday::Tuesday => Day::Second,
            time::Weekday::Wednesday => Day::Third,
            time::Weekday::Thursday => Day::Four,
            time::Weekday::Friday => Day::Five,
            time::Weekday::Saturday => Day::Six,
            time::Weekday::Sunday => Day::Seven,
        }
    }
}

impl PartialEq<Day> for TimeCellRepiter {
    fn eq(&self, other: &Day) -> bool {
        match self {
            TimeCellRepiter::Regular(day) => day == other,
            TimeCellRepiter::Once(day, _) => day == other,
        }
    }
}
