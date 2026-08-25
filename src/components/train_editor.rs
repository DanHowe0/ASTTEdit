use dioxus::prelude::*;
use crate::models::{Train, TrainData};

#[component]
pub fn TrainEditor(
    train: Signal<Option<Train>>,
    on_update: EventHandler<(String, TrainData)>,
) -> Element {
    let original_train: Train = match train.read().clone() {
        Some(t) => t,
        None => {
            return rsx! {
                div {
                    class: "train-editor",

                    div {
                        class: "editor-header",

                        h2 {
                            "Train Editor"
                        }

                        p {
                            "Editing Train: None"
                        }
                    }
                }
            };
        }
    };

    let mut edit_train: Signal<Train> = use_signal(|| original_train.clone());
    if edit_train.read().id() == "NEW" {edit_train.write().set_id("2A00".to_string())}
    use_effect(move || {
        if let Some(new_train) = train.read().clone() {
            edit_train.set(new_train);
        }
    });

    rsx! {
        div {
            class: "train-editor",

            div {
                class: "editor-header",

                h2 {
                    "Train Editor"
                }

                p {
                    "Editing Train: {original_train.id()}"
                }
            }

            div {
                class: "editor-section",

                h3 {
                    "Basic Information"
                }

                div {
                    class: "editor-field",

                    label {
                        "Train ID"
                    }

                    input {
                        value: "{edit_train.read().id()}",

                        oninput: move |event| {
                            let mut train = edit_train.write().set_id(event.value());
                        }
                    }
                }

                div {
                    class: "editor-field",

                    label {
                        "Entry Time"
                    }

                    input {
                        value: "{edit_train.read().arrival()}",

                        oninput: move |event| {
                            let mut train = edit_train.write().set_arrival(event.value());
                        }
                    }
                }

                div {
                    class: "editor-field",

                    label {
                        "Exit Time"
                    }

                    input {
                        value: "{edit_train.read().departure()}",

                        oninput: move |event| {
                            let mut train = edit_train.write().set_departure(event.value());
                        }
                    }
                }
            }

            div {
                class: "editor-section",

                h3 {
                    "Route"
                }

                div {
                    class: "route-selection",

                    div {
                        class: "editor-field",

                        label {
                            "Direction"
                        }

                        select {
                            onchange: move |event| {
                                let path = edit_train.read().get_path().to_string();
                                let mut train = edit_train.write().set_path(event.value(), path);
                            },

                            value: "{edit_train.read().direction()}",

                            option {
                                value: "Down",
                                "Down"
                            }

                            option {
                                value: "Up",
                                "Up"
                            }
                        }
                    }

                    div {
                        class: "editor-field",

                        label {
                            "Path"
                        }

                        select {
                            onchange: move |event| {
                                let dir = edit_train.read().direction().to_string();
                                let mut train = edit_train.write().set_path(dir, event.value());
                            },

                            value: "{edit_train.read().get_path()}",

                            option {
                                value: "Main",
                                "Main"
                            }

                            option {
                                value: "Branch",
                                "Branch"
                            }
                        }
                    }
                }

                div {
                    class: "route-description",

                    span {
                        "{edit_train.read().from()}"
                    }

                    span {
                        " → "
                    }

                    span {
                        "{edit_train.read().destination()}"
                    }
                }
            }

            div {
                class: "editor-actions",

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