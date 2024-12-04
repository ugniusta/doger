use std::collections::HashMap;
use std::rc::Rc;
use chrono::NaiveDate;
use crate::components::modals;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::components::timetable::{Timetable, timetable};
use crate::models::Dog;
use crate::server_fns;

pub fn Home() -> Element {
    let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2024, 3, 10).unwrap();

    let mut dogs = use_signal(move || None);
    let mut kennels = use_signal(move || None);
    let mut stays = use_signal(move || None);

    use_future(move || async move {
        dogs.set(server_fns::get_dogs().await.ok())
    });
    use_future(move || async move {
        kennels.set(server_fns::get_kennels().await.ok())
    });
    use_future(move || async move {
        stays.set(server_fns::get_stays_between(start_date, end_date).await.ok());
    });

    let mut layout = use_memo(move || {
        let stays = &*stays.read();
        if let Some(stays) = stays.as_ref() {
            Some(timetable::compute_layout(stays, start_date, end_date))
        } else { None }
    });

    let mut modal_is_shown = use_signal(move || false);

    rsx! {
        Timetable { layout, kennels, dogs }

        modals::StayRegistrationModal { show: modal_is_shown, dogs, kennels }
        modals::Button { show: !modal_is_shown(), onclick: move |_| modal_is_shown.set(true) }
    }
}
