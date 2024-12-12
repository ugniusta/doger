use std::collections::{HashSet};
use std::str::FromStr;
use chrono::NaiveDate;
use dioxus::document::eval;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::components::buttons::{Button1, Button2, Button3};
use crate::components::input_fields::TextInput;
use crate::components::pages::home::Resources;
use crate::models::{Dog, Kennel, Stay, TimeslotIndex};
use crate::server_fns;

#[derive(Props, Clone, PartialEq)]
pub struct StayRegistrationModalProps {
    onclose: EventHandler<()>,
    dogs: ReadOnlySignal<Option<Vec<Dog>>>,
    kennels: ReadOnlySignal<Option<Vec<Kennel>>>,
}

pub fn StayRegistrationModal(mut props: StayRegistrationModalProps) -> Element {
    let mut is_hidden = use_signal(|| false);
    let mut resources = use_context::<Resources>();
    let mut status_messages = use_signal(|| None);

    let mut dog_name = use_signal(|| "".to_owned());
    let mut kennel_name = use_signal(|| "".to_owned());
    let mut start_date = use_signal(|| "".to_owned());
    let mut end_date = use_signal(|| "".to_owned());

    let mut clear_data = move || {
        dog_name.set("".to_owned());
        kennel_name.set("".to_owned());
        start_date.set("".to_owned());
        end_date.set("".to_owned());
        status_messages.set(None);
    };

    let mut close_modal = move || {
        clear_data();
        props.onclose.call(());
    };

    async fn pick_date(mut date: Signal<String>) {
        let timeslot_index = get_timeslot_index().await;
        date.set(timeslot_index.date.to_string());
    }

    let pick_start_date = move |_| async move {
        is_hidden.set(true);
        pick_date(start_date).await;
        is_hidden.set(false);
    };
    let pick_end_date = move |_| async move {
        is_hidden.set(true);
        pick_date(end_date).await;
        is_hidden.set(false);
    };

    let add_stay = move || async move {
        let mut valid = true;
        let mut errors = Vec::new();

        let dog_id = match i32::from_str(&*dog_name.read()) {
            Ok(val) => Some(val),
            Err(err) => {
                errors.push("Invalid dog id".to_owned());
                None
            }
        };
        let kennel_id = match i32::from_str(&*kennel_name.read()) {
            Ok(val) => Some(val),
            Err(err) => {
                errors.push("Invalid kennel id".to_owned());
                None
            }
        };
        let start_date = match NaiveDate::parse_from_str(&*start_date.read(), "%Y-%m-%d") {
            Ok(val) => Some(val),
            Err(err) => {
                errors.push("Invalid start date".to_owned());
                None
            }
        };
        let end_date = match NaiveDate::parse_from_str(&*end_date.read(), "%Y-%m-%d") {
            Ok(val) => Some(val),
            Err(err) => {
                errors.push("Invalid end date".to_owned());
                None
            }
        };

        if let (Some(start_date), Some(end_date)) = (start_date, end_date) {
            if start_date > end_date {
                errors.push("End date must not be before start date".to_owned());
            }
        }

        if errors.is_empty() {
            let stay = Stay {
                id: None,
                dog_id: dog_id.unwrap(),
                kennel_id: kennel_id.unwrap(),
                start_date: start_date.unwrap(),
                end_date: end_date.unwrap(),
            };

            match server_fns::insert_stay(stay).await {
                Ok(_) => {
                    resources.fetch_stays.restart();
                    props.onclose.call(());
                }
                Err(_) => {
                    let message = format!("Error inserting stay: {:?}", "Database error :(");
                    status_messages.set(Some(vec![message]))
                }
            }
        } else {
            status_messages.set(Some(errors));
        }
    };

    rsx! {
        div {
            class: if is_hidden() { "hidden" },
            class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center",

            div { class: "min-w-1/3 flex flex-col space-y-2 bg-white rounded px-4 py-4",
                div {
                    label { class: "block", "Dog" }
                    TextInput { value: dog_name, placeholder: "Enter a dog ID" }
                }
                div {
                    label { class: "block", "Kennel" }
                    TextInput { value: kennel_name, placeholder: "Enter a kennel ID" }
                }
                div {
                    label { class: "block", "Start date" }
                    TextInput { value: start_date, placeholder: "Select or enter a start date" }
                    button { onclick: pick_start_date, "SELECT" }
                }
                div {
                    label { class: "block", "End date" }
                    TextInput { value: end_date, placeholder: "Select or enter an end date" }
                    button { onclick: pick_end_date, "SELECT" }
                }

                div {
                    if let Some(messages) = &*status_messages.read() {
                        for message in messages {
                            p { class: "text-red-500", "{message}" }
                        }
                    }
                }

                div { class: "flex justify-between",
                    div {
                        Button1 { text: "Cancel", onclick: move |_| close_modal() }
                    }
                    div {
                        Button2 { text: "Add", onclick: move |_| add_stay() }
                    }
                }
            }
        }
    }
}

pub async fn get_timeslot_index() -> TimeslotIndex {
    let received = eval(include_str!("../get_timeslot_index.js")).recv().await;
    serde_json::from_value(received.unwrap()).unwrap()
}

