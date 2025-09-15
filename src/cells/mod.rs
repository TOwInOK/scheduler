use std::fmt::Display;

use cell::{Cell, day::Day, groups::Groups};
use serde::{Deserialize, Serialize};
use time::{Date, Duration, Month};

pub mod cell;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cells<'a> {
    #[serde(borrow)]
    pub cells: Vec<Cell<'a>>,
}

impl<'a> Cells<'a> {
    pub fn append(&mut self, mut other: Self) {
        self.cells.append(&mut other.cells);
    }
    pub fn filter(&self, group: Groups, date: Date) -> Self {
        let day = Into::<Day>::into(date.weekday());
        Cells {
            cells: self
                .cells
                .iter()
                .copied()
                .filter(|cell| cell.day == day)
                .filter(|cell| cell.group_allowed == group)
                .filter(|cell| cell.subject.show(date))
                .filter(|cell| cell.odd == is_academic_week_odd(date))
                .collect::<Vec<Cell<'a>>>(),
        }
    }
    pub fn sort(&mut self) {
        self.cells.sort_by_key(|x| x.para);
    }

    pub fn filter_and_sort(&self, group: Groups, date: Date) -> Self {
        let mut f = self.filter(group, date);
        f.sort();
        f
    }

    pub fn filtered_week(&self, group: Groups, date: Date) -> Vec<(Date, Self)> {
        let days_from_monday = date.weekday().number_days_from_monday();
        let monday = date - Duration::days(days_from_monday as i64);

        (0..=6).fold(Vec::new(), |mut acc: Vec<(Date, Self)>, v: u8| {
            let date = monday + Duration::days(v as i64);
            acc.push((date, self.filter_and_sort(group, date)));
            acc
        })
    }
}

impl Display for Cells<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.cells
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

pub fn is_academic_week_even(current_date: Date) -> bool {
    let current_year = current_date.year();

    // Определяем дату начала "учебного года" (1 сентября).
    let academic_year_start_date = if current_date.month() < Month::September {
        Date::from_calendar_date(current_year - 1, Month::September, 1)
    } else {
        Date::from_calendar_date(current_year, Month::September, 1)
    }
    .expect("Failed to create academic year start date");

    // Находим понедельник недели, на которой находится 1 сентября.
    // Это будет началом первой учебной недели.
    let days_from_monday = academic_year_start_date.weekday().number_days_from_monday();
    let first_academic_monday = academic_year_start_date - Duration::days(days_from_monday as i64);

    // Вычисляем количество полных недель между началом учебного года и текущей датой.
    // Добавляем 1, чтобы первая неделя была "неделей 1".
    let academic_week_number = (current_date - first_academic_monday).whole_weeks() as u32 + 1;

    // Проверяем четность номера недели.
    academic_week_number % 2 == 0
}

pub fn is_academic_week_odd(current_date: Date) -> bool {
    !is_academic_week_even(current_date)
}

// pub fn get_week_boundaries(date: OffsetDateTime) -> (OffsetDateTime, OffsetDateTime) {
//     let sunday = date
//         .checked_sub(Duration::days(date.weekday().number_from_monday() as i64))
//         .unwrap();
//     let sunday = sunday.replace_time(time::Time::MIDNIGHT);
//     let saturday = sunday
//         .checked_add(Duration::days(6))
//         .unwrap()
//         .replace_time(time::Time::from_hms(23, 59, 59).unwrap());
//     (sunday, saturday)
// }

/// Render header
pub fn header(date: Date, group: Groups) -> String {
    format!(
        "{}, {} ({}) — {}\n{}",
        group,
        Into::<Day>::into(date.weekday()).short_display(),
        if is_academic_week_odd(date) {
            "Н"
        } else {
            "Ч"
        },
        date,
        "=".repeat(23)
    )
}

/// Just render header + days in cells
///
/// **note**: does't filter cells.
/// Do it manualy
pub fn render_cells(cells: &Cells, group: Groups, date: Date) -> String {
    let mut buffer = String::new();
    buffer.push_str(&header(date, group));
    buffer.push('\n');
    for cell in cells.cells.iter() {
        buffer.push_str(&cell.to_string());
        buffer.push('\n');
    }
    buffer
}

pub fn render_cells_week(cells: &[(Date, Cells)], group: Groups) -> String {
    let mut buffer = String::new();
    for (date, cells) in cells {
        buffer.push_str(&header(*date, group));
        buffer.push('\n');
        for cell in cells.cells.iter() {
            buffer.push_str(&cell.to_string());
            buffer.push('\n');
        }
    }
    buffer
}
