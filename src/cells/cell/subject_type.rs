use crate::dicts::{
    lection_dict::{
        LECTION_ALGEBRA_END, LECTION_ALGEBRA_START, LECTION_BIO_END, LECTION_BIO_START,
        LECTION_DEVICE_END, LECTION_DEVICE_START, LECTION_HISTORY_END, LECTION_HISTORY_START,
        LECTION_MATH_ANALYZ_END, LECTION_MATH_ANALYZ_START, LECTION_OFFICE_END,
        LECTION_OFFICE_START, LECTION_OS_END, LECTION_OS_START, LECTION_PROGRAMMING_END,
        LECTION_PROGRAMMING_START,
    },
    practice_dict::{
        PRACTICE_ALGEBRA_END, PRACTICE_ALGEBRA_START, PRACTICE_BIO_END, PRACTICE_BIO_START,
        PRACTICE_DEVICE_END, PRACTICE_DEVICE_START, PRACTICE_ENGLISH_END, PRACTICE_ENGLISH_START,
        PRACTICE_HISTORY_END, PRACTICE_HISTORY_START, PRACTICE_MATH_ANALYZ_END,
        PRACTICE_MATH_ANALYZ_START, PRACTICE_OFFICE_END, PRACTICE_OFFICE_START, PRACTICE_OS_END,
        PRACTICE_OS_START, PRACTICE_PROGRAMMING_END, PRACTICE_PROGRAMMING_START,
        PRACTICE_PROJECT_END, PRACTICE_PROJECT_START, PRACTICE_SPORT_END, PRACTICE_SPORT_START,
    },
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use time::Date;

use super::subject::Subject;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SubjectType {
    Lection(Subject),
    Practice(Subject),
}

impl Display for SubjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubjectType::Lection(_) => write!(f, "Лекция"),
            SubjectType::Practice(_) => write!(f, "Практика"),
        }
    }
}

impl SubjectType {
    pub fn subject(&self) -> Subject {
        match self {
            SubjectType::Lection(subject) => *subject,
            SubjectType::Practice(subject) => *subject,
        }
    }

    /// Returns true if the subject is available at the given time.
    pub fn show(&self, date: Date) -> bool {
        match self {
            SubjectType::Lection(subject) => match subject {
                Subject::English => false,
                Subject::History => date >= LECTION_HISTORY_START && date <= LECTION_HISTORY_END,
                Subject::Algebra => date >= LECTION_ALGEBRA_START && date <= LECTION_ALGEBRA_END,
                Subject::MathAnalyz => {
                    date >= LECTION_MATH_ANALYZ_START && date <= LECTION_MATH_ANALYZ_END
                }
                Subject::OfficeAndTechnology => {
                    date >= LECTION_OFFICE_START && date <= LECTION_OFFICE_END
                }
                Subject::Sport => false,
                Subject::OSBasics => date >= LECTION_OS_START && date <= LECTION_OS_END,
                Subject::DeviceAndFunction => {
                    date >= LECTION_DEVICE_START && date <= LECTION_DEVICE_END
                }
                Subject::Biologia => date >= LECTION_BIO_START && date <= LECTION_BIO_END,
                Subject::Programming => {
                    date >= LECTION_PROGRAMMING_START && date <= LECTION_PROGRAMMING_END
                }
                Subject::Project => false,
                Subject::Curator => false,
            },
            SubjectType::Practice(subject) => match subject {
                Subject::English => date >= PRACTICE_ENGLISH_START && date <= PRACTICE_ENGLISH_END,
                Subject::History => date >= PRACTICE_HISTORY_START && date <= PRACTICE_HISTORY_END,
                Subject::Algebra => date >= PRACTICE_ALGEBRA_START && date <= PRACTICE_ALGEBRA_END,
                Subject::MathAnalyz => {
                    date >= PRACTICE_MATH_ANALYZ_START && date <= PRACTICE_MATH_ANALYZ_END
                }
                Subject::OfficeAndTechnology => {
                    date >= PRACTICE_OFFICE_START && date <= PRACTICE_OFFICE_END
                }
                Subject::Sport => date >= PRACTICE_SPORT_START && date <= PRACTICE_SPORT_END,
                Subject::OSBasics => date >= PRACTICE_OS_START && date <= PRACTICE_OS_END,
                Subject::DeviceAndFunction => {
                    date >= PRACTICE_DEVICE_START && date <= PRACTICE_DEVICE_END
                }
                Subject::Biologia => date >= PRACTICE_BIO_START && date <= PRACTICE_BIO_END,
                Subject::Programming => {
                    date >= PRACTICE_PROGRAMMING_START && date <= PRACTICE_PROGRAMMING_END
                }
                Subject::Project => date >= PRACTICE_PROJECT_START && date <= PRACTICE_PROJECT_END,
                Subject::Curator => false,
            },
        }
    }
}
