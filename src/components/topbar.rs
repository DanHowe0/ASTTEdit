use dioxus::prelude::*;

#[component]
pub fn Topbar(
    on_new: EventHandler<()>,
    on_open: EventHandler<()>,
//    on_save: EventHandler<()>,
    on_save_as: EventHandler<()>,
) -> Element {
    let mut file_open = use_signal(|| false);

    rsx! {
        div {
            class: "topbar",

            div {
                class: "menu",

                button {
                    class: "menu-button",

                    onclick: move |_| {
                        file_open.toggle();
                    },

                    "File"
                }

                if file_open() {
                    div {
                        class: "dropdown",

                        button {
                            class: "dropdown-item",

                            onclick: move |_| {
                                file_open.set(false);
                                on_new.call(());
                            },

                            "New Timetable"
                        }

                        button {
                            class: "dropdown-item",

                            onclick: move |_| {
                                file_open.set(false);
                                on_open.call(());
                            },

                            "Open..."
                        }

                        //button {
                        //    class: "dropdown-item",

                        //    onclick: move |_| {
                        //        file_open.set(false);
                        //        on_save.call(());
                        //    },

                        //    "Save"
                        //}

                        button {
                            class: "dropdown-item",

                            onclick: move |_| {
                                file_open.set(false);
                                on_save_as.call(());
                            },

                            "Save As..."
                        }
                    }
                }
            }
        }
    }
}