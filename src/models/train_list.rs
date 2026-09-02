use crate::models::{Train, TrainData};
use crate::services::file_loader::LoadFile;
use std::path::PathBuf;
use dioxus::prelude::*;

fn valid_time(time: &str) -> bool {
    if time.len() != 5 {
        return false;
    }

    let parts: Vec<&str> = time.split(':').collect();

    if parts.len() != 2 {
        return false;
    }

    let hours: u32 = match parts[0].parse() {
        Ok(value) => value,
        Err(_) => return false,
    };

    let minutes: u32 = match parts[1].parse() {
        Ok(value) => value,
        Err(_) => return false,
    };

    // Valid clock time
    if hours > 23 || minutes > 59 {
        return false;
    }

    // Timetable range: 08:00 - 23:59
    hours >= 8
}

pub struct TrainList {
    pub file: PathBuf,
    pub trains: Vec<Train>,
    pub dirty: bool,
}

impl TrainList {
    pub fn empty(file: PathBuf) -> Self {
        Self {
            file,
            trains: Vec::new(),
            dirty: false,
        }
    }

    pub fn new(file: PathBuf) -> Result<Self, String> {
        let data = LoadFile(&file)?;

        let trains = data
            .into_iter()
            .map(Train::new)
            .collect();

        let mut list = Self {
            file,
            trains,
            dirty: false,
        };

        list.order("arrival");

        Ok(list)
    }

    pub fn save_as(&mut self, path: PathBuf) -> Result<(), String> {
        let data: Vec<&TrainData> = self.trains
            .iter()
            .map(|train| train.data())
            .collect();

        let json = serde_json::to_string_pretty(&data)
            .map_err(|error| format!("Failed to serialize timetable: {}", error))?;

        std::fs::write(&path, json)
            .map_err(|error| format!("Failed to write timetable: {}", error))?;

        self.file = path;
        self.dirty = false;

        Ok(())
    }

    pub fn order(&mut self, by: &str) -> Vec<Train> {

        let mut trains = self.trains.clone();

        if by == "arrival" { trains.sort_by(|a,b| a.arrival().cmp(b.arrival()));}

        self.trains = trains.clone();
        trains
    }

    pub fn load(&mut self, file: PathBuf) -> Result<(), String> {
        let data = LoadFile(&file)?;

        self.file = file;

        self.trains = data
            .into_iter()
            .map(Train::new)
            .collect();

        self.dirty = false;

        Ok(())
    }

    pub fn reload(&mut self) -> Result<(), String> {
        let data = LoadFile(&self.file)?;

        self.trains = data
            .into_iter()
            .map(Train::new)
            .collect();

        self.dirty = false;

        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), String> {
        let data: Vec<TrainData> = Vec::new();
        self.trains = data.into_iter().map(Train::new).collect();

        Ok(())
    }

    pub fn trains(&self) -> &[Train] {
        &self.trains
    }

    pub fn has_id(&self, id: &str) -> bool {
        self.trains
            .iter()
            .any(|train| train.id() == id)
    }

    pub fn delete_train(&mut self, id: &str) -> bool {
        self.trains.retain(|train| train.id() != id);
        self.dirty = true;
        return true
    }

    pub fn update_train(&mut self, original_id: &str, new_data: TrainData) -> Result<(), String> {

    let id: &String = &new_data.id;

    let valid_format =
        id.len() == 4
        && id.chars().nth(0).unwrap().is_ascii_digit()
        && id.chars().nth(1).unwrap().is_ascii_uppercase()
        && id.chars().nth(2).unwrap().is_ascii_digit()
        && id.chars().nth(3).unwrap().is_ascii_digit();


    if !valid_format {
        return Err(format!(
            "Train ID '{}' is not in the required format 0X00.",
            id
        ));
    }

    if !valid_time(&new_data.arrival_time) {
        return Err(format!(
            "Arrival time '{}' must be format xx:xx and be between 08:00 and 23:59.",
            new_data.arrival_time
        ));
    }

    if !valid_time(&new_data.departure_time) {
        return Err(format!(
            "Departure time '{}' must be format xx:xx and be between 08:00 and 23:59.",
            new_data.departure_time
        ));
    }

    if original_id != new_data.id && self.has_id(&new_data.id) {
        return Err(format!(
            "Train ID '{}' already exists.",
            new_data.id
        ));
    }

    if original_id == "NEW TRAIN" {
        let train: Train = Train::new(new_data);
        self.trains.push(train);
        self.order("arrival");
        self.dirty = true;
        return Ok(());
    }

    let train = self.trains.
        iter_mut()
        .find(|train| train.id() == original_id)
        .ok_or_else(|| {
            format!(
                "Train '{}' was not found.",
                original_id
            )
        })?;

    train.update(new_data);

    self.order("arrival");

    self.dirty = true;

    Ok(())
}
}