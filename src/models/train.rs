use serde::{Deserialize, Serialize};

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
    pub fn set_id(&mut self, val: String) {
        self.data.id = val;
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