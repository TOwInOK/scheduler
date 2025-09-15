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
    info!("Stard loading store");
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
            load(&path).await.inspect_err(|x| {
                tracing::error!("Error reading file: {}\n\tError: ->{}", path, x)
            })?,
        );
    }
    info!("Load is done");
    Ok(cells)
}
