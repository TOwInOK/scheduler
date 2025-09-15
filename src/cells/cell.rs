pub mod day;
pub mod groups;
pub mod para;
pub mod subgroup;
pub mod subject;
pub mod subject_type;
pub mod time;

use std::fmt::{self};

use ::time::format_description;
use groups::Groups;
use para::Para;
use serde::{Deserialize, Serialize};
use subject_type::SubjectType;
use time::TimeCellRepiter;

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
/// Representation of a cell in the schedule.
pub struct Cell<'a> {
    pub subject: SubjectType,
    pub place: &'a str,
    pub day: TimeCellRepiter,
    pub para: Para,
    pub odd: bool,
    pub group_allowed: Groups,
}

impl fmt::Display for Cell<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Форматируем время
        let hm = format_description::parse("[hour]:[minute]").unwrap();
        let m = format_description::parse("[minute]").unwrap();
        let (start, end, break_time) = self.para.time();
        let start = start.format(&hm).unwrap();
        let end = end.format(&hm).unwrap();
        let break_time = break_time.format(&m).unwrap();

        // Иконка типа предмета
        let icon = match self.subject {
            SubjectType::Lection(_) => '🟠',
            SubjectType::Practice(_) => '🟢',
        };

        // Формируем строки
        let line1 = format!("{}. {}–{} {}", self.para, start, end, icon);
        let line2 = format!("{}", self.subject.subject());
        let line3 = format!("📍{} | Перерыв: {} мин", self.place, break_time);

        // Пишем все строки с пустой строкой после пары
        writeln!(f, "{}", line1)?;
        writeln!(f, "{}", line2)?;
        writeln!(f, "{}", line3)
    }
}
