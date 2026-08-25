use dioxus::{desktop::WindowBuilder, prelude::*};
use std::env;
use std::path::PathBuf;

const MAIN_CSS: &str = include_str!("../assets/main.css");

mod models;
mod services;
mod components;

use components::{TrainEditor, TrainSelector, Topbar};
use crate::models::{Train, TrainData, TrainList};
use crate::services::notification::show_error;

fn main() {
    LaunchBuilder::desktop()
        .with_cfg(
            dioxus::desktop::Config::new()
                .with_menu(None)
                .with_window(
                    WindowBuilder::new()
                        .with_always_on_top(false)
                        .with_title("ASTTE")
                )
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    let appdata: String = env::var("APPDATA").expect("APPDATA not found");
    let fileName: String = "timetable_triang.json".to_string();
    let filePath: PathBuf = PathBuf::from(format!(r"{}\WhitePawGames\timetables\", appdata));
    let file: PathBuf = filePath.clone().join(fileName);
    
    let load_trains: TrainList = match TrainList::new(file.clone()) {
        Ok(list) => list,
        Err(error) => {
            println!("Failed to load timetable: {}\n{}", error, file.display());
            return rsx! {
                p {
                    "Failed to load timetable: {error}"
                }
            }
        }
    };

    let mut train_list: Signal<TrainList> = use_signal(|| load_trains);
    let mut selected_train: Signal<Option<Train>> = use_signal(|| None::<Train>);

    let load = filePath.clone();
    let save = filePath.clone();

    const DISABLE_CONTEXT_MENU: &str = r#"
        document.addEventListener('contextmenu', function(event) {
            event.preventDefault();
        });
    "#;

    rsx! {
        document::Style {
            {MAIN_CSS}
        }


        document::Script {
            {DISABLE_CONTEXT_MENU}
        }


        div {
            class: "app",
        

            Topbar {
                on_new: move |_| {
                    train_list.write().clear();
                },

                on_open: move |_| {
                    let directory = load.clone();

                    spawn(async move {
                        let path = tokio::task::spawn_blocking(move || {
                            rfd::FileDialog::new()
                                .add_filter("Timetable", &["json"])
                                .set_directory(directory)
                                .pick_file()
                        })
                        .await
                        .ok()
                        .flatten();

                        if let Some(path) = path {
                            match TrainList::new(path) {
                                Ok(new_list) => {
                                    train_list.set(new_list);
                                    selected_train.set(None);
                                }

                                Err(error) => {
                                    show_error(&error);
                                }
                            }
                        }
                    });
                },

                //on_save: move |_| {
                //    println!("Save");
                //},

                on_save_as: move |_| {
                    let directory = save.clone();

                    spawn(async move {
                        let path = tokio::task::spawn_blocking(move || {
                            rfd::FileDialog::new()
                                .add_filter("Timetable", &["json"])
                                .set_directory(directory)
                                .set_file_name("timetable.json")
                                .save_file()
                        })
                        .await
                        .ok()
                        .flatten();

                        if let Some(path) = path {
                            match train_list.write().save_as(path) {
                                Ok(()) => {
                                    println!("Timetable saved successfully.");
                                }

                                Err(error) => {
                                    show_error(&error);
                                }
                            }
                        }
                    });
                },
            }


            div {
                class: "editor-layout",

                TrainSelector {
                    train_list,
                    selected_train,
                }

                TrainEditor {
                    // Pass the Train wrapper object herec
                    train: selected_train,

                    on_update: move |(original_id, new_data): (String, TrainData)| {
                        let new_id = new_data.id.clone();

                        let result = train_list
                            .write()
                            .update_train(
                                &original_id,
                                new_data,
                            );

                        match result {
                            Ok(()) => {
                                let updated_train = train_list
                                    .read()
                                    .trains()
                                    .iter()
                                    .find(|train| train.id() == new_id)
                                    .cloned();

                                selected_train.set(updated_train);
                            }

                            Err(error) => {
                                show_error(&error);
                            }
                        }
                    }
                }
            }
        }
    }
}
