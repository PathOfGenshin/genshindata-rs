/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LunaRiteBattleBuffExcelConfigData = Vec<LunaRiteBattleBuffExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LunaRiteBattleBuffExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "EOEIOLGBMGM")]
    pub eoeiolgbmgm: String,
    #[serde(rename = "MGMKCENCEEG")]
    pub mgmkcenceeg: i64,
    #[serde(rename = "JMAMOBDKECP")]
    pub jmamobdkecp: i64,
    #[serde(rename = "rewardId")]
    pub reward_id: Option<i64>,
}
