// use super::timeslot::{DOMRect, TimeslotId};
//
// #[derive(Clone, PartialEq)]
// pub struct TimeslotSelection {
//     pub start_id: Option<TimeslotId>,
//     pub end_id: Option<TimeslotId>,
//     pub start_point: Option<(f64, f64)>,
//     pub end_point: Option<(f64, f64)>,
// }
//
// impl TimeslotSelection {
//     pub fn new() -> Self {
//         TimeslotSelection {
//             start_id: None,
//             end_id: None,
//             start_point: None,
//             end_point: None,
//         }
//     }
//
//     // TODO - unfinished
//     pub fn select_cell(&mut self, id: &TimeslotId, rect: DOMRect) {
//         match (&self.start_id, &self.end_id) {
//             (None, _) => {
//                 self.start_id = Some(id.clone());
//                 let (start_x, start_y) = (rect.x + rect.scroll_left, rect.y + rect.scroll_top);
//                 self.start_point = Some((start_x, start_y));
//                 self.end_point = Some((start_x + rect.width, start_y + rect.height));
//             }
//             (Some(_), None) => {
//                 self.end_id = Some(id.clone());
//                 self.end_point = Some((rect.x + rect.width + rect.scroll_left,
//                                        rect.y + rect.height + rect.scroll_top))
//             }
//             (Some(_), Some(_)) => {
//                 self.start_id = None;
//                 self.start_point = None;
//                 self.end_id = None;
//                 self.end_point = None;
//             }
//         }
//     }
// }
