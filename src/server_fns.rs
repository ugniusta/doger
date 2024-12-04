use std::collections::HashMap;
use chrono::{Duration, NaiveDate};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::models::*;
#[cfg(feature = "server")]
use crate::database;


type Result<T> = std::result::Result<T, ServerFnError>;

#[server]
pub async fn get_dogs() -> Result<Vec<Dog>> {
    let FromContext(pool) = extract().await?;
    let dogs = database::get_dogs(&pool).await?;
    Ok(dogs)
}

#[server]
pub async fn insert_dog(dog: Dog) -> Result<i32> {
    let FromContext(pool) = extract().await?;
    let id = database::insert_dog(&pool, dog).await?;
    Ok(id)
}

#[server]
pub async fn get_kennels() -> Result<Vec<Kennel>> {
    let FromContext(pool) = extract().await?;
    let kennels = database::get_kennels(&pool).await?;
    Ok(kennels)
}

#[server]
pub async fn insert_stay(stay: Stay) -> Result<i32> {
    let FromContext(pool) = extract().await?;
    let id = database::insert_stay(&pool, stay).await?;
    Ok(id)
}

#[server]
pub async fn get_stays_between(start_date: NaiveDate, end_date: NaiveDate) -> Result<Vec<Stay>> {
    let FromContext(pool) = extract().await?;
    let mut stays = database::get_stays_between(&pool, start_date, end_date).await?;
    Ok(stays)
}

