use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct InputProps {
    value: Signal<String>,
    placeholder: String,
}
pub fn TextInput(mut props: InputProps) -> Element {
    rsx! {
        input {
            class: "px-2 py-2 border border-gray-900 rounded focus:outline-none focus:ring-2 focus:ring-blue-500",
            placeholder: props.placeholder,
            value: "{props.value}",
            oninput: move |e| props.value.set(e.value())
        }
    }
}