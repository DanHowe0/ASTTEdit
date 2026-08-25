use dioxus::prelude::*;
use crate::models::Train;

#[component]
pub fn TrainRow(
    train: Train,
    selected: bool,
    on_select: EventHandler<Train>,
) -> Element {

    rsx! {
        tr {
            class: if selected { "selected" } else { "" },

            onclick: move |_| {
                on_select.call(train.clone());
            },

            td {
                "{train.data.arrival_time}"
            }

            td {
                "{train.data.id}"
            }

            td {
                "{train.data.destination}"
            }

            td {
                "{train.data.from_instrument}"
            }
        }
    }
}