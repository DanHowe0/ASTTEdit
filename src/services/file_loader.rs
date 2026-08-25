use crate::models::{Train, TrainData};
use std::fs;
use std::path::Path;


pub fn LoadFile(path: &Path) -> Result<Vec<TrainData>, String> {
    let contents = fs::read_to_string(path)
        .map_err(|error| error.to_string())?;

    let trains: Vec<TrainData> = serde_json::from_str(&contents)
        .map_err(|error| error.to_string())?;

    Ok(trains)
}