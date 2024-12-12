use dioxus::html::textarea::placeholder;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::components::buttons::{Button1, Button3};
use crate::components::input_fields::TextInput;

#[derive(Props, PartialEq, Clone)]
pub struct BottomMenuProps {
    onclose: EventHandler<()>,
}
pub fn BottomMenu(props: BottomMenuProps) -> Element {
    let mut stay_id = use_signal(|| "".to_owned());
    rsx! {
        div { class: "fixed bottom-0 w-full, h-1/3 bg-white",

            TextInput { value: stay_id, placeholder: "ID" }
            div { class: "flex justify-between",
                Button1 { text: "Cancel", onclick: props.onclose }
                Button3 { text: "Delete", onclick: move |_| info!("Closed!") }
            }
        }
    }
}

