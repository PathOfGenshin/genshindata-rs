/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ReunionV2SignInExcelConfigData = Vec<ReunionV2SignInExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReunionV2SignInExcelConfigDatum {
    #[serde(rename = "BHOJKBHBKFN")]
    pub bhojkbhbkfn: i64,
    pub day_index: i64,
    pub reward_id: i64,
}