/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityGcgpvePuzzleRewardExcelConfigData = Vec<ActivityGcgpvePuzzleRewardExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityGcgpvePuzzleRewardExcelConfigDatum {
    pub id: i64,
    pub challenge_id: i64,
    pub watcher_id: i64,
}
