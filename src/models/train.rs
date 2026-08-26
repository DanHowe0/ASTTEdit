use dioxus::html::g::class;
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct TrainData {
    pub id: String,
    pub path: String,
    pub progress: f64,
    pub speed: f64,
    pub arrival_time: String,
    pub departure_time: String,
    pub destination: String,
    pub destination_progress: String,
    pub from_instrument: String,
    pub bell_code: Vec<i32>,
}

pub fn get_bell_code(train_class: &str, empty: bool) -> Vec<i32> {
    match (train_class, empty) {
        ("0", _) => vec![2, 3],
        ("1", _) => vec![4],
        ("2", _) => vec![3, 1],

        ("3", _) => vec![1, 3, 1],
        ("4", _) => vec![3, 1, 1],
        ("5", _) => vec![2, 2, 1],
        ("6", _) => vec![5],
        ("7", _) => vec![4, 1],
        ("8", _) => vec![3, 2],

        ("9", false) => vec![1, 4],
        ("9", true) => vec![1, 4, 1],

        _ => vec![],
    }
}

pub fn get_class_from_bell_code(bell_code: &[i32]) -> (&'static str, bool) {
    match bell_code {
        [2, 3] => ("0", false),
        [4] => ("1", false),
        [3, 1] => ("2", false),

        [1, 3, 1] => ("3", false),
        [3, 1, 1] => ("4", false),
        [2, 2, 1] => ("5", false),
        [5] => ("6", false),
        [4, 1] => ("7", false),
        [3, 2] => ("8", false),

        [1, 4] => ("9-passenger", false),
        [1, 4, 1] => ("9-empty", true),

        _ => ("", false),
    }
}

#[derive(Clone, PartialEq)]
pub struct Train {
    pub data: TrainData,
}

impl Train {
    pub fn new(data: TrainData) -> Self {
        Self {
            data,
        }
    }

    pub fn update(&mut self, new_data: TrainData) {
        self.data = new_data;
    }

    pub fn data(&self) -> &TrainData {
        &self.data
    }

    pub fn id(&self) -> &str {
        &self.data.id
    }

    pub fn train_class(&self) -> &str {
        let (t_class, empty): (&str, bool) = get_class_from_bell_code(&self.data.bell_code);
        return t_class;
    }

    pub fn bell_code(&self) -> &Vec<i32> {
        &self.data.bell_code
    }

    pub fn set_train_class(&mut self, t_class: &str, empty: bool) {
        self.data.id.replace_range(0..1, t_class);
        self.data.bell_code = get_bell_code(t_class, empty);
    }

    pub fn letter(&self) -> &str {
        &self.data.id[1..2]
    }

    pub fn set_letter(&mut self, t_class: &str) {
        self.data.id.replace_range(1..2, t_class);
    }

    pub fn number(&self) -> &str {
        &self.data.id[2..4]
    }

    pub fn set_number(&mut self, t_class: &str) {
        self.data.id.replace_range(2.., t_class);
    }


    pub fn arrival(&self) -> &str {
        &self.data.arrival_time
    }
    pub fn set_arrival(&mut self, val: String) {
        self.data.arrival_time = val;
    } 

    pub fn departure(&self) -> &str {
        &self.data.departure_time
    }
    pub fn set_departure(&mut self, val: String) {
        self.data.departure_time = val;
    }

    pub fn destination(&self) -> &str {
        &self.data.destination
    }
    pub fn set_destination(&mut self, val: String) {
        self.data.destination = val;
    }

    pub fn from(&self) -> &str {
        &self.data.from_instrument
    }
    pub fn set_from(&mut self, val: String) {
        self.data.from_instrument = val;
    }

    pub fn path(&self) -> &str {
        &self.data.path
    }

    pub fn get_path(&self) -> &str {
        if self.data.from_instrument == "Doortown" || self.data.destination == "Doortown" {
            "Main"
        } else {
            "Branch"
        }
    }

    pub fn set_path(&mut self, dir: String, path: String) {
        let (path, from, destination) = match (dir.as_str(), path.as_str()) {
            ("Down", "Main") => (
                "DownMain",
                "Chippinhall",
                "Doortown",
            ),

            ("Down", "Branch") => (
                "DownMain",
                "Chippinhall",
                "Goton",
            ),

            ("Up", "Main") => (
                "UpMain",
                "Doortown",
                "Chippinhall",
            ),

            ("Up", "Branch") => (
                "UpBranch",
                "Goton",
                "Chippinhall",
            ),

            _ => return,
        };

        self.data.path = path.to_string();
        self.data.from_instrument = from.to_string();
        self.data.destination = destination.to_string();
    }

    pub fn direction(&self) -> &str {
        if self.data.from_instrument == "Chippinhall" {
            "Down"
        } else {
            "Up"
        }
    }

}