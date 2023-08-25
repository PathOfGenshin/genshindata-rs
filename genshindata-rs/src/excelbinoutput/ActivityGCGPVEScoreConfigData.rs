/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityGcgpveScoreConfigData = Vec<ActivityGcgpveScoreConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityGcgpveScoreConfigDatum {
    pub id: i64,
    #[serde(rename = "GODINNKAGLG")]
    pub godinnkaglg: String,
    pub level_name_text_map_hash: i64,
    pub score: Option<i64>,
}
