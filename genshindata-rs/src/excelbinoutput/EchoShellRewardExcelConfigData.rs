/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type EchoShellRewardExcelConfigData = Vec<EchoShellRewardExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct EchoShellRewardExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "rewardId")]
    pub reward_id: i64,
    pub eiobkgcagli: i64,
    pub ackaabacbmk: Option<bool>,
}
