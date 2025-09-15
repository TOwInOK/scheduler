use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Day {
    First,
    Second,
    Third,
    Four,
    Five,
    Six,
    Seven,
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Day::First => write!(f, "Понедельник"),
            Day::Second => write!(f, "Вторник"),
            Day::Third => write!(f, "Среда"),
            Day::Four => write!(f, "Четверг"),
            Day::Five => write!(f, "Пятница"),
            Day::Six => write!(f, "Суббота"),
            Day::Seven => write!(f, "Воскресенье"),
        }
    }
}

impl Day {
    pub fn short_display(&self) -> &'static str {
        match self {
            Day::First => "Пн",
            Day::Second => "Вт",
            Day::Third => "Ср",
            Day::Four => "Чт",
            Day::Five => "Пт",
            Day::Six => "Сб",
            Day::Seven => "Вс",
        }
    }
}

impl From<u8> for Day {
    fn from(value: u8) -> Self {
        match value {
            0 => Day::First,
            1 => Day::Second,
            2 => Day::Third,
            3 => Day::Four,
            4 => Day::Five,
            5 => Day::Six,
            6 => Day::Seven,
            _ => panic!("Invalid day value"),
        }
    }
}
