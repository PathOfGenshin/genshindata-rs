/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type QuestEventsConfigData = Vec<QuestEventsConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct QuestEventsConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub ppfocdhmmem: Vec<i64>,
    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,
    pub iehhomnpfll: Option<f64>,
    pub mahnheodgip: i64,
    pub jhmiandobio: Option<f64>,
}
