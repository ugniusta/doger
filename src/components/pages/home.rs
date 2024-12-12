use chrono::NaiveDate;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

use crate::components;
use components::{modals, buttons, bottom_menu, timetable};
use bottom_menu::BottomMenu;
use timetable::{Timetable, compute_layout};
use modals::StayRegistrationModal;
use buttons::Button1;
use crate::components::input_fields::TextInput;
use crate::server_fns;

#[derive(Clone, Copy)]
pub struct Resources {
    pub fetch_dogs: UseFuture,
    pub fetch_kennels: UseFuture,
    pub fetch_stays: UseFuture,
}

impl Resources {
    pub fn fetch_all(&mut self) {
        self.fetch_dogs.restart();
        self.fetch_kennels.restart();
        self.fetch_stays.restart();
    }
}

pub fn Home() -> Element {
    let mut start_date = use_signal(move || chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
    let mut end_date = use_signal(move || chrono::NaiveDate::from_ymd_opt(2024, 3, 31).unwrap());

    let mut dogs = use_signal(move || None);
    let mut kennels = use_signal(move || None);
    let mut stays = use_signal(move || None);

    let mut fetch_dogs = use_future(move || async move {
        dogs.set(server_fns::get_dogs().await.ok());
    });
    let mut fetch_kennels = use_future(move || async move {
        kennels.set(server_fns::get_kennels().await.ok());
    });
    let mut fetch_stays = use_future(move || async move {
        stays.set(server_fns::get_stays_between(start_date(), end_date()).await.ok());
    });

    let mut resources = use_context_provider(move || {
        Resources {
            fetch_dogs,
            fetch_kennels,
            fetch_stays,
        }
    });

    let mut layout = use_memo(move || {
        let stays = &*stays.read();
        let kennels = &*kennels.read();

        if let (Some(stays), Some(kennels)) = (stays.as_ref(), kennels.as_ref()) {
            Some(timetable::compute_layout(stays, kennels, start_date(), end_date()))
        } else { None }
    });

    let mut stay_modal_is_open = use_signal(move || false);
    let mut bottom_menu_is_open = use_signal(move || false);

    let mut filter_start = use_signal(move || start_date().to_string());
    let mut filter_end = use_signal(move || end_date().to_string());

    rsx! {
        div { class: "flex flex-col justify-between w-full h-screen",
            div { class: "overflow-y-auto",
                Timetable { layout, kennels, dogs }
            }
            div {
                class: if stay_modal_is_open() || bottom_menu_is_open() { "hidden" },
                class: "flex h-auto justify-between p-4 w-full border border-gray-900",
                div { class: "flex space-x-2",
                    div {
                        Button1 {
                            text: "Add Stay",
                            onclick: move |_| {
                                stay_modal_is_open.set(true);
                            }
                        }
                    }
                    div {
                        Button1 {
                            text: "Delete Stay",
                            onclick: move |_| {
                                bottom_menu_is_open.set(true);
                            }
                        }
                    }
                }
                div { class: "flex space-x-2 flex-nowrap",
                    div { class: "max-w-[100px]",
                        TextInput { value: filter_start, placeholder: "Start" }
                    }
                    div { class: "max-w-[100px]",
                        TextInput { value: filter_end, placeholder: "End" }
                    }
                    Button1 {
                        text: "Filter",
                        onclick: move |_| {
                            if let (Ok(start), Ok(end)) = (
                                NaiveDate::parse_from_str(&*filter_start.read(), "%Y-%m-%d"),
                                NaiveDate::parse_from_str(&*filter_end.read(), "%Y-%m-%d"),
                            ) {
                                if start <= end {
                                    start_date.set(start);
                                    end_date.set(end);
                                    fetch_stays.restart();
                                }
                            }
                        }
                    }
                }
            }
        }
        div { class: if !stay_modal_is_open() { "hidden" },
            StayRegistrationModal {
                dogs,
                kennels,
                onclose: move |_| {
                    stay_modal_is_open.set(false);
                }
            }
        }
        div { class: if !bottom_menu_is_open() { " hidden" },
            BottomMenu {
                onclose: move |_| {
                    bottom_menu_is_open.set(false);
                }
            }
        }
    }
}
