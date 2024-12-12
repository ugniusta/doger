use std::fmt::Write;
use std::str::FromStr;
use dioxus::html::textarea::placeholder;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::components::buttons::{Button1, Button3};
use crate::components::input_fields::TextInput;
use crate::components::pages::home::Resources;
use crate::server_fns;

#[derive(Props, PartialEq, Clone)]
pub struct BottomMenuProps {
    onclose: EventHandler<()>,
}
pub fn BottomMenu(props: BottomMenuProps) -> Element {
    let mut resources = use_context::<Resources>();
    let mut stay_id = use_signal(|| "".to_owned());
    let mut message = use_signal(|| "".to_owned());

    let mut clear_data = move || {
        stay_id.write().clear();
        message.write().clear();
    };

    let close_menu = move || {
        props.onclose.call(());
        clear_data();
    };

    let delete_stay = move || async move {
        let stay_id = i32::from_str(&*stay_id.read());
        message.write().clear();
        if let Ok(stay_id) = stay_id {
            if let Err(e) = server_fns::delete_stay(stay_id).await {
                info!("{:?}", e);
                message.write().write_str("Error deleting stay");
            } else {
                resources.fetch_stays.restart();
            }
        } else {
            message.write().write_str("Invalid stay ID");
        }
    };

    rsx! {
        div { class: "fixed p-4 bottom-0 w-full, h-1/3 bg-white",

            TextInput { value: stay_id, placeholder: "ID" }
            div { class: "flex justify-between",
                Button1 { text: "Cancel", onclick: close_menu }
                Button3 { text: "Delete", onclick: delete_stay }
            }
            if !(&*message.read()).is_empty() {
                p { class: "text-red-500", "ⓧ {message}" }
            }
        }
    }
}

