use std::collections::{HashSet};
use std::rc::Rc;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::models::{Dog, Kennel, Stay};
use crate::server_fns;

#[component]
pub fn Button(show: bool, onclick: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: if !show { "hidden" },
            class: "bg-green bg-opacity-50 fixed bottom-3 right-3",
            onclick: move |_| onclick.call(()),
            "+ Stay"
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct StayRegistrationModalProps {
    show: Signal<bool>,
    dogs: ReadOnlySignal<Option<Vec<Dog>>>,
    kennels: ReadOnlySignal<Option<Vec<Kennel>>>,
}

pub fn StayRegistrationModal(mut props: StayRegistrationModalProps) -> Element {
    let mut status_message = use_signal(|| "".to_string());

    let mut show = props.show;
    let dogs = &*props.dogs.read();
    let kennels = &*props.kennels.read();

    let mut dog_name = use_signal(|| "".to_string());
    let mut kennel_name = use_signal(|| "".to_string());
    let mut start_date = use_signal(|| "".to_string());
    let mut end_date = use_signal(|| "".to_string());

    let set_dog_name = move |event: Event<FormData>| {
        dog_name.set(event.value())
    };
    let set_kennel_name = move |event: Event<FormData>| {
        kennel_name.set(event.value())
    };
    let set_start_date = move |event: Event<FormData>| {
        start_date.set(event.value())
    };
    let set_end_date = move |event: Event<FormData>| {
        end_date.set(event.value())
    };
    let add_stay = move |_| async move {
        let dog_id = 1;
        let kennel_id = 1;
        let stay = Stay {
            id: None,
            dog_id,
            kennel_id,
            start_date: chrono::NaiveDate::from_ymd_opt(2024, 01, 01).unwrap(),
            end_date: chrono::NaiveDate::from_ymd_opt(2024, 01, 05).unwrap(),
        };

        match server_fns::insert_stay(stay).await {
            Ok(_) => status_message.set("Stay created successfully!".to_string()),
            Err(e) => status_message.set(format!("Error inserting stay: {:?}", e)),
        }
    };

    rsx! {
        div {
            class: if !show() { "hidden" },
            class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center",
            div { class: "flex flex-col space-y-2",
                div {
                    label { class: "block", "Dog" }
                    input {
                        list: "dog_input_suggestions",
                        value: "{dog_name}",
                        oninput: set_dog_name
                    }
                    datalist { id: "dog_input_suggestions",
                        if let Some(dogs) = &*props.dogs.read() {
                            for dog in dogs.iter() {
                                option { value: "{dog.name}" }
                            }
                        }
                    }
                }
                div {
                    label { class: "block", "Kennel" }
                    input { value: "{kennel_name}", oninput: set_kennel_name }
                }
                div {
                    label { class: "block", "Start date" }
                    input { value: "{start_date}", oninput: set_start_date }
                }
                div {
                    label { class: "block", "End date" }
                    input { value: "{end_date}", oninput: set_end_date }
                }
                div { class: "block text-right",
                    button { class: "bg-white-500", onclick: add_stay, "Add" }
                }
                div { class: "block",
                    button { onclick: move |_| show.set(false), "Close" }
                }

                div { class: "block", "Status: {status_message()}" }
            }
        }
    }
}
