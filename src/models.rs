use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize, Serializer};
use chrono::{DateTime, NaiveDate, Utc};
use serde::ser::{SerializeMap, SerializeStruct};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct DogStay {
    pub dog: String,
    pub id: i32,
    pub start_day: i32,
    pub end_day: i32,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct KennelWithStays {
    pub name: String,
    pub dog_stays: Vec<DogStay>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Dog {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Kennel {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Stay {
    pub id: Option<i32>,
    pub dog_id: i32,
    pub kennel_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct TimeslotData {
    pub stay_id: i32,
}

pub type KennelID = i32;
pub type ColumnID = u32;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct TimeslotIndex(pub NaiveDate, pub KennelID, pub ColumnID);
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct TimetableLayout {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub timeslot_data: HashMap<TimeslotIndex, TimeslotData>,
    pub column_counts: HashMap<KennelID, u32>,
}

impl From<(NaiveDate, KennelID, ColumnID)> for TimeslotIndex {
    fn from(value: (NaiveDate, KennelID, ColumnID)) -> Self {
        TimeslotIndex(value.0, value.1, value.2)
    }
}