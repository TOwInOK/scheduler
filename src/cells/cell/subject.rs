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
            Subject::English => write!(f, "Иностранный язык"),
            Subject::History => write!(f, "Основы российской государтсвенности"),
            Subject::Algebra => write!(f, "Линейная алгебра"),
            Subject::MathAnalyz => write!(f, "Математический анализ"),
            Subject::OfficeAndTechnology => write!(f, "Офисные приложения и технологии"),
            Subject::Sport => write!(f, "Физкультура"),
            Subject::OSBasics => write!(f, "Основы операционных систем"),
            Subject::DeviceAndFunction => write!(f, "Устройства и функционирование соврем-х ИС"),
            Subject::Biologia => write!(f, "Биологические основы"),
            Subject::Programming => write!(f, "Основы программирования"),
            Subject::Project => write!(f, "Проектная деятельность"),
            Subject::Curator => write!(f, "Кураторский час"),
        }
    }
}
