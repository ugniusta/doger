use dioxus::prelude::*;

#[component]
pub fn Button1(text: String, onclick: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "bg-blue-300 py-2 px-4 rounded",
            onclick: move |_| onclick.call(()),
            "{text}"
        }
    }
}

#[component]
pub fn Button2(text: String, onclick: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "bg-green-500 py-2 px-4 rounded",
            onclick: move |_| onclick.call(()),
            "{text}"
        }
    }
}

#[component]
pub fn Button3(text: String, onclick: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "bg-red-500 py-2 px-4 rounded",
            onclick: move |_| onclick.call(()),
            "{text}"
        }
    }
}
