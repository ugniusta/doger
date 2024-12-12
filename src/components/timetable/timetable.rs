use std::collections::HashMap;
use std::rc::Rc;
use chrono::{Datelike, Duration, NaiveDate};
use dioxus::document::eval;
use dioxus::html::completions::CompleteWithBraces::col;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use serde::{Deserialize, Serialize};

use crate::models::*;
use super::timeslot::{Timeslot};

#[derive(Clone, PartialEq, Props)]
pub struct TimetableProps {
    layout: ReadOnlySignal<Option<TimetableLayout>>,
    kennels: ReadOnlySignal<Option<Vec<Kennel>>>,
    dogs: ReadOnlySignal<Option<Vec<Dog>>>,
}

// TODO: Test date edge cases.
fn generate_month_dates(start_date: NaiveDate, end_date: NaiveDate) -> Vec<(i32, u32, Vec<NaiveDate>)> {
    let mut months = Vec::new();

    let mut current_date = start_date;
    while current_date <= end_date {
        let current_year = current_date.year();
        let current_month = current_date.month();
        let mut month_days = Vec::with_capacity(31);
        while current_date <= end_date && current_date.month() == current_month {
            month_days.push(current_date);
            current_date += Duration::days(1);
        }
        months.push((current_year, current_month, month_days));
    }
    months
}

fn find_free_kennel_column(
    timeslot_data: &HashMap<TimeslotIndex, TimeslotData>,
    stay: &Stay,
    kennel_id: KennelID,
) -> u32 {
    let mut kennel_column = 0;
    'outer: loop {
        let mut current_date = stay.start_date;
        while current_date <= stay.end_date {
            let index = TimeslotIndex::from((current_date, kennel_id, kennel_column));
            if timeslot_data.get(&index).is_some() {
                kennel_column += 1;
                continue 'outer;
            }
            current_date += Duration::days(1);
        }
        return kennel_column;
    }
}

pub fn compute_layout(stays: &Vec<Stay>, kennels: &Vec<Kennel>, start_date: NaiveDate, end_date: NaiveDate) -> TimetableLayout {
    let mut layout = TimetableLayout {
        timeslot_data: HashMap::new(),
        column_counts: HashMap::new(),
        start_date,
        end_date,
    };

    let mut timeslot_data = HashMap::new();
    let mut column_counts: HashMap<KennelID, u32> = HashMap::new();
    for stay in stays {
        let kennel_id = stay.kennel_id;

        let kennel_column = find_free_kennel_column(&timeslot_data, &stay, kennel_id);
        column_counts
            .entry(kennel_id)
            .and_modify(|e| { *e = (*e).max(kennel_column + 1); })
            .or_insert(1);

        let mut current_date = stay.start_date;
        while current_date <= stay.end_date {
            let index = TimeslotIndex::from((current_date, kennel_id, kennel_column));
            timeslot_data.insert(index, TimeslotData {
                stay_id: stay.id
                    .unwrap(),
                color: "#e28743".to_owned(),
            });
            current_date += Duration::days(1);
        }
    }

    for kennel in kennels {
        column_counts
            .entry(kennel.id.unwrap_or(0))
            .or_insert(1);
    }
    layout.timeslot_data = timeslot_data;
    layout.column_counts = column_counts;

    layout
}

pub fn Timetable(props: TimetableProps) -> Element {
    let layout = (&*props.layout.read());
    let kennels = (&*props.kennels.read());

    if let (Some(layout), Some(_)) = (layout.as_ref(), kennels.as_ref()) {
        let timeslot_data = &layout.timeslot_data;
        let mut kennel_column_counts = layout.column_counts
            .iter()
            .map(|(k, c)| (*k, *c))
            .collect::<Vec<(i32, u32)>>();
        let start_date = layout.start_date;
        let end_date = layout.end_date;

        kennel_column_counts.sort_by(|a, b| a.0.cmp(&b.0));
        let month_dates = generate_month_dates(start_date, end_date);

        let layout = props.layout.map(move |v| v.as_ref().unwrap());
        let kennels = props.kennels.map(move |v| v.as_ref().unwrap());

        rsx! {
            table { class: "w-full table-fixed",

                for (year , month , dates) in month_dates {
                    tbody { "year-month": "{year}-{month}",
                        for date in dates {
                            tr { "day": "{date.day()}",
                                th { class: "w-16 border border-blue-500 font-semibold",
                                    p { "{date.year()}-{date.month()}" }
                                    p { "{date.day()}" }
                                }
                                for (kennel_id , column_count) in kennel_column_counts.iter() {
                                    for column in 0..(*column_count) {
                                        Timeslot {
                                            kennel_id: *kennel_id,
                                            column,
                                            data: {
                                                let index = TimeslotIndex::from((date, *kennel_id, column));
                                                if let Some(data) = timeslot_data.get(&index) {
                                                    Some(data.clone())
                                                } else {
                                                    None
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! { "Loading timetable..." }
    }
}

#[component]
fn TimetableHeader(layout: MappedSignal<TimetableLayout>, kennels: MappedSignal<Vec<Kennel>>) -> Element {
    let layout = &*layout.read();
    let kennels = &*kennels.read();

    rsx! {
        table { class: "w-full table-fixed",
            thead {
                tr {
                    th { class: "w-16 border border-blue-500 font-semibold", "Date" }
                    for kennel in kennels.iter() {
                        th { class: "border border-blue-500", colspan: { *(layout.column_counts.get(&kennel.id.unwrap()).unwrap()) }, {kennel.name.clone()} }
                    }
                }
            }
        }
    }
}

// #[component]
// pub fn TimetableSelector(selection: Signal<TimeslotSelection>, border_width: f64) -> Element {
//     let selection = selection.read();
//     let (width, height) = match (selection.start_point, selection.end_point) {
//         (Some((x_start, y_start)), Some((x_end, y_end))) => {
//             (x_end - x_start /* - border_width*/,
//              y_end - y_start /*- border_width*/)
//         }
//         (_, _) => (0.0, 0.0)
//     };
//     let (left, top) = selection.start_point.unwrap();
//
//     rsx! {
//         div {
//             style: r#"width: {width}px;
// 	   	              height: {height}px;
// 	   	              top: {top}px;
// 	   	              left: {left}px;
// 	   	              position: absolute;
// 	   	              pointer-events: none;"#,
//             class: "border border-red-500"
//         }
//     }
// }

