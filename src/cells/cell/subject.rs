use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Enum representing different subjects.
pub enum Subject {
    English,
    History,
    Algebra,
    MathAnalyz,
    OfficeAndTechnology,
    Sport,
    OSBasics,
    DeviceAndFunction,
    Biologia,
    Programming,
    Project,
    Curator,
}
impl Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Subject::English => write!(f, "Английский"),
            Subject::History => write!(f, "История"),
            Subject::Algebra => write!(f, "Алгебра"),
            Subject::MathAnalyz => write!(f, "Математический анализ"),
            Subject::OfficeAndTechnology => write!(f, "Офис и технологии"),
            Subject::Sport => write!(f, "Физкультура"),
            Subject::OSBasics => write!(f, "Основы ОС"),
            Subject::DeviceAndFunction => write!(f, "Устройства и функции"),
            Subject::Biologia => write!(f, "Биология"),
            Subject::Programming => write!(f, "Программирование"),
            Subject::Project => write!(f, "Проект"),
            Subject::Curator => write!(f, "Куратор"),
        }
    }
}
