use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use sqlx::prelude::*;
use sqlx::{PgPool, query, query_as, Result};
use crate::models::*;
use chrono::NaiveDate;

pub async fn connect(url: String) -> Result<PgPool> {
    PgPool::connect(&url).await
}

pub async fn get_dogs(pool: &PgPool) -> Result<Vec<Dog>> {
    query_as!(
        Dog, r"
        SELECT * FROM Dogs
        "
    ).fetch_all(pool).await
}

pub async fn insert_dog(pool: &PgPool, dog: Dog) -> Result<i32> {
    let result = query!(r"
        INSERT INTO Dogs (name)
        VALUES ($1)
        RETURNING id
        ",
        dog.name
    ).fetch_one(pool).await?;
    Ok(result.id)
}

pub async fn get_kennels(pool: &PgPool) -> Result<Vec<Kennel>> {
    query_as!(
        Kennel, r"
        SELECT * FROM Kennels
        "
    ).fetch_all(pool).await
}

pub async fn get_stays(pool: &PgPool) -> Result<Vec<Stay>> {
    query_as!(
        Stay, r"
        SELECT * FROM Stays
        "
    ).fetch_all(pool).await
}

pub async fn insert_stay(pool: &PgPool, stay: Stay) -> Result<i32> {
    let result = query!(r"
        INSERT INTO Stays (dog_id, kennel_id, start_date, end_date)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        ",
        stay.dog_id,
        stay.kennel_id,
        stay.start_date,
        stay.end_date
    ).fetch_one(pool).await?;
    Ok(result.id)
}

pub async fn get_stays_between(pool: &PgPool, start_date: NaiveDate, end_date: NaiveDate) -> Result<Vec<Stay>> {
    query_as!(
        Stay, r"
        SELECT * FROM Stays
        WHERE start_date <= $1 AND end_date >= $2
        ",
        end_date,
        start_date
    ).fetch_all(pool).await
}