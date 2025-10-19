use std::path::Path;

use tokio::fs;
use tracing::{info, instrument};

use crate::{cells::Cells, error::Result};

pub async fn load(path: &str) -> Result<Cells<'static>> {
    let buffer: &'static mut [u8] = fs::read(&path).await?.leak();
    ron::de::from_bytes::<Cells<'static>>(buffer).map_err(|x| x.into())
}

pub async fn save(path: impl AsRef<Path>, data: &Cells<'_>) -> Result<()> {
    let file = ron::ser::to_string_pretty(data, ron::ser::PrettyConfig::new())?;
    fs::write(&path, file).await.map_err(|x| x.into())
}

#[instrument]
pub async fn load_cells_store() -> Result<Cells<'static>> {
    info!("Start loading store");
    let store_names = [
        "algebra",
        "biologia",
        "device_and_function",
        "english",
        "history",
        "math_analyz",
        "office",
        "os_basics",
        "programming",
        "project",
        "sport",
    ];
    let mut cells = Cells { cells: Vec::new() };
    for path in store_names.map(|x| format!("./store/{}.ron", x)) {
        cells.append(
            load(&path).await.inspect_err(
                |x| tracing::error!(path = %path, error = %x, "Error of reading file"),
            )?,
        );
    }
    info!("Load is done");
    Ok(cells)
}

#[cfg(test)]
mod tests {
    use arrayvec::ArrayVec;
    use time::macros::date;
    use tracing::Level;

    use crate::{
        cells::cell::{Cell, subject_type::SubjectType, time::TimeCellRepiter},
        init_logger,
    };

    use super::*;

    #[tokio::test]
    async fn check_time_format() {
        let cell = Cell {
            subject: SubjectType::Lection(crate::cells::cell::subject::Subject::Algebra),
            place: "TEST",
            day: TimeCellRepiter::Regular(crate::cells::cell::day::Day::First),
            para: crate::cells::cell::para::Para::Five,
            odd: false,
            groups_allowed: ArrayVec::new(),
            start_at: Some(date!(2025 - 02 - 25)),
            end_at: None,
        };
        let cells = vec![cell];
        let cells = Cells { cells };
        save("./test.ron", &cells).await.unwrap();
    }

    #[tokio::test]
    async fn parse_store() {
        init_logger(Level::INFO);
        load_cells_store().await.unwrap();
    }
}
