#![allow(non_snake_case)]
#![allow(unused)]

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
pub mod models;
pub mod server_fns;
pub mod components;

#[cfg(feature = "server")]
pub mod database;

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("Failed to init logger.");
    LaunchBuilder::new()
        .with_context(server_only!(
            tokio::runtime::Runtime::new()
                .expect("Failed to initialize Tokio")
                .block_on({
                    let database_url = std::env::var("DATABASE_URL")
                        .expect("DATABASE_URL not set");
                    database::connect(database_url)
                })
                .expect("Database connection failed")
        ))
        .launch(app)
}

pub fn app() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        components::pages::Home {}
    }
}