use std::collections::HashMap;
use std::rc::Rc;
use dioxus::document::eval;
use dioxus::prelude::*;
use serde::Deserialize;

use crate::models::*;


#[derive(Props, PartialEq, Clone)]
pub struct TimeslotProps {
    kennel_id: KennelID,
    column: u32,
    data: Option<TimeslotData>,
}

pub fn Timeslot(mut props: TimeslotProps) -> Element {
    let baseline_style = "border border-gray-900";
    if let Some(data) = props.data {
        rsx! {
            td {
                "kennel_id": "{props.kennel_id}",
                "column": "{props.column}",
                class: "{baseline_style}",
                style: "background-color: {data.color}",
                "{data.stay_id}"
            }
        }
    } else {
        rsx! {
            td {
                "kennel_id": "{props.kennel_id}",
                "column": "{props.column}",
                class: "{baseline_style}",
                ""
            }
        }
    }

    // let mut selection = props.selection;
    // let select_timeslot = move |_| {
    //     let id = id.clone();
    //     async move {
    //         let rect = get_cell_coordinates(&id).await;
    //         selection.write().select_cell(&id, rect);
    //     }
    // };
}

// // TODO: possibly move out
// #[derive(Deserialize, Debug)]
// pub(crate) struct DOMRect {
//     pub x: f64,
//     pub y: f64,
//     pub width: f64,
//     pub height: f64,
//     pub scroll_top: f64,
//     pub scroll_left: f64,
// }

// async fn get_cell_coordinates(cell_id: &TimeslotId) -> DOMRect {
//     let cell_css_id = cell_id.to_css_id();
//     let mut eval = eval(&format!(r#"
//         const cell = document.getElementById({cell_css_id});
//         const rect = cell.getBoundingClientRect();
//         const scrollTop = window.pageYOffset;
//         const scrollLeft = window.pageXOffset;
//         dioxus.send({{
//             scroll_top: scrollTop,
//             scroll_left: scrollLeft,
//             x: rect.x,
//             y: rect.y,
//             width: rect.width,
//             height: rect.height}})
//         "#,
//     ));
//     let received = eval.recv().await;
//     serde_json::from_value(received.unwrap()).unwrap()
// }
