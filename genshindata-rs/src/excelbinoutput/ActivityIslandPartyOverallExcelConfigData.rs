/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityIslandPartyOverallExcelConfigData = Vec<ActivityIslandPartyOverallExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ActivityIslandPartyOverallExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "activityId")]
    pub activity_id: i64,
    pub ijkglbhpjbb: i64,
    pub bkiafphehdc: i64,
}
