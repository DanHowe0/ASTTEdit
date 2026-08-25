use dioxus::prelude::*;

use crate::models::{Train, TrainList, TrainData};

use super::TrainRow;

#[component]
pub fn TrainSelector(
    train_list: Signal<TrainList>,
    selected_train: Signal<Option<Train>>,
) -> Element {
    rsx! {
        div {
            class: "train-selector",
            div{
                class: "train-list",
                table {

                    thead {
                        tr {
                            th { "Time" }
                            th { "ID" }
                            th { "Destination" }
                            th { "From" }
                        }
                    }

                    tbody {
                        for train in train_list.read().trains().iter() {

                            TrainRow {
                                train: train.clone(),

                                selected: selected_train
                                    .with(|selected| selected.as_ref().map(|t| t.id() == train.id()).unwrap_or(false)),

                                on_select: move |train: Train| {
                                    selected_train.set(
                                        Some(train)
                                    );
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "train-menu",
                button {
                    onclick: move |_| {
                        let data = TrainData {
                            id: "NEW".to_string(),
                            path: "DownMain".to_string(),
                            progress: 0.0,
                            speed: 0.2,
                            arrival_time: "08:00".to_string(),
                            departure_time: "08:13".to_string(),
                            destination: "Doortown".to_string(),
                            destination_progress: "end".to_string(),
                            from_instrument: "Chippinhall".to_string(),
                            bell_code: vec![3, 1],
                        };

                        selected_train.set(
                            Some(Train::new(data))
                        );
                    },

                    "Add Entry"
                }
                button {
                    onclick: move |_| {
                                            
                        let original_train: Train = match selected_train.read().clone() {
                            Some(t) => t,
                            None => {return}
                        };

                        let deleted: bool = train_list.write().delete_train(original_train.id());
                        if deleted {selected_train.set(None)}
                    },

                    "Delete Entry"
                }
            }
        }
    }
}