use dioxus::prelude::*;
use crate::models::{Train, TrainData};

#[component]
pub fn TrainEditor( train: Signal<Option<Train>>, on_update: EventHandler<(String, TrainData)> ) -> Element {


    let original_train: Train = match train.read().clone() {
        Some(t) => t,
        None => {
            return rsx! {
                div { class: "train-editor",

                    div { class: "editor-header",

                        h2 { "Train Editor" }

                        p { "Editing Train: None" }
                    }
                }
            };
        }
    };

    let mut edit_train: Signal<Train> = use_signal(|| original_train.clone());
    if edit_train.read().id() == "NEW TRAIN" {
        edit_train.write().set_train_class("2", false);
        edit_train.write().set_letter("A");
        edit_train.write().set_number("00");
    }
    use_effect(move || {
        if let Some(new_train) = train.read().clone() {
            edit_train.set(new_train);
        }
    });

    let bell_code: String = edit_train.read()
                                    .bell_code()
                                    .iter()
                                    .map(|n| n.to_string())
                                    .collect::<Vec<_>>()
                                    .join("-");

    rsx! {
        div { class: "train-editor",

            div { class: "editor-header",

                h2 { "Train Editor" }

                p { "Editing Train: {original_train.id()}" }
            }

            div { class: "editor-section",

                h3 { "Headcode Creator" }

                div { class: "editor-field",

                    label { "Train Class" }

                    select {
                        value: "{edit_train.read().train_class()}",

                        oninput: move |event| {
                            let value = event.value();

                            let (class, empty) = match value.as_str() {
                                "0" => ("0", false),
                                "1" => ("1", false),
                                "2" => ("2", false),
                                "3" => ("3", false),
                                "4" => ("4", false),
                                "5" => ("5", false),
                                "6" => ("6", false),
                                "7" => ("7", false),
                                "8" => ("8", false),
                                "9-passenger" => ("9", false),
                                "9-empty" => ("9", true),

                                _ => return,
                            };

                            edit_train
                                .write()
                                .set_train_class(class, empty);
                        },

                        option { value: "0", "Class 0 - Passenger" }

                        option { value: "1", "Class 1 - Passenger" }

                        option { value: "2", "Class 2 - Passenger" }

                        option { value: "3", "Class 3 - Freight" }

                        option { value: "4", "Class 4 - Freight" }

                        option { value: "5", "Class 5 - Freight" }

                        option { value: "6", "Class 6 - Freight" }

                        option { value: "7", "Class 7 - Freight" }

                        option { value: "8", "Class 8 - Freight" }

                        option { value: "9-passenger", "Class 9 - Passenger" }

                        option { value: "9-empty", "Class 9 - Empty" }
                    }
                }

                div { class: "editor-field",

                    label { "Letter" }

                    select {
                        value: "{edit_train.read().letter()}",

                        oninput: move |event| {
                            edit_train
                                .write()
                                .set_letter(&event.value());
                        },

                        option { value: "A", "A" }
                        option { value: "B", "B" }
                        option { value: "C", "C" }
                        option { value: "D", "D" }
                        option { value: "E", "E" }
                        option { value: "F", "F" }
                        option { value: "G", "G" }
                        option { value: "H", "H" }
                        option { value: "J", "J" }
                        option { value: "K", "K" }
                        option { value: "L", "L" }
                        option { value: "M", "M" }
                        option { value: "N", "N" }
                        option { value: "P", "P" }
                        option { value: "Q", "Q" }
                        option { value: "R", "R" }
                        option { value: "S", "S" }
                        option { value: "T", "T" }
                        option { value: "U", "U" }
                        option { value: "V", "V" }
                        option { value: "W", "W" }
                        option { value: "X", "X" }
                        option { value: "Y", "Y" }
                        option { value: "Z", "Z" }
                    }
                }

                div { class: "editor-field",

                    label { "Number" }

                    input {
                        r#type: "number",
                        min: "0",
                        max: "99",
                        value: "{edit_train.read().number()}",

                        oninput: move |event| {
                            if let Ok(number) = event.value().parse::<u8>() {
                                let number = number.min(99);
                                let formatted = format!("{:02}", number);

                                edit_train
                                    .write()
                                    .set_number(&formatted);
                            }
                        },
                    }
                }

                div { class: "editor-field",

                    label { "ID: {edit_train.read().id()}" }
                
                }
                div { class: "editor-field",

                    label { "Bell Code: {bell_code}" }
                
                }
            }

            div { class: "editor-section",

                h3 { "Timing" }

                div { class: "editor-field",

                    label { "Entry Time" }

                    input {
                        value: "{edit_train.read().arrival()}",

                        oninput: move |event| {
                            let mut train = edit_train.write().set_arrival(event.value());
                        },
                    }
                }

                div { class: "editor-field",

                    label { "Exit Time" }

                    input {
                        value: "{edit_train.read().departure()}",

                        oninput: move |event| {
                            let mut train = edit_train.write().set_departure(event.value());
                        },
                    }
                }
            }

            div { class: "editor-section",

                h3 { "Route" }

                div { class: "route-selection",

                    div { class: "editor-field",

                        label { "Direction" }

                        select {
                            onchange: move |event| {
                                let path = edit_train.read().get_path().to_string();
                                let mut train = edit_train.write().set_path(event.value(), path);
                            },

                            value: "{edit_train.read().direction()}",

                            option { value: "Down", "Down" }
                            option { value: "Up", "Up" }
                        }
                    }

                    div { class: "editor-field",

                        label { "Path" }

                        select {
                            onchange: move |event| {
                                let dir = edit_train.read().direction().to_string();
                                let mut train = edit_train.write().set_path(dir, event.value());
                            },

                            value: "{edit_train.read().get_path()}",

                            option { value: "Main", "Main" }
                            option { value: "Branch", "Branch" }
                        }
                    }
                }

                div { class: "route-description",

                    span { "{edit_train.read().from()}" }

                    span { " → " }

                    span { "{edit_train.read().destination()}" }
                }
            }

            div { class: "editor-actions",

                button {
                    onclick: move |_| {
                        let new_data = edit_train.read().data.clone();
                        let original_id = original_train.id().to_string();
                        let result = on_update.call((original_id, new_data));

                    },

                    "Apply Changes"
                }
            }
        }
    }
}