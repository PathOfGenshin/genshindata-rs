/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityMistTrialAvatarDataExcelConfigData = Vec<ActivityMistTrialAvatarDataExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMistTrialAvatarDataExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,
    pub schedule_id: i64,
    pub trial_avatar_id: i64,
}
