/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type TrialAvatarActivityExcelConfigData = Vec<TrialAvatarActivityExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TrialAvatarActivityExcelConfigDatum {
    pub schedule_id: i64,
    pub avatar_index_id_list: Vec<i64>,
    pub reward_id_list: Vec<i64>,
}
