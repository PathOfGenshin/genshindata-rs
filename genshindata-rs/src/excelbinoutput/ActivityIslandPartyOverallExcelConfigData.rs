/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityIslandPartyOverallExcelConfigData = Vec<ActivityIslandPartyOverallExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityIslandPartyOverallExcelConfigDatum {
    pub id: i64,
    pub activity_id: i64,
    pub card_display_countdown: i64,
    pub score_display_countdown: i64,
}
