/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type WinterCampBattleExcelConfigData = Vec<WinterCampBattleExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WinterCampBattleExcelConfigDatum {
    pub id: i64,
    pub open_day: i64,
    pub priority: i64,
    #[serde(rename = "PGGIJCDNCAM")]
    pub pggijcdncam: i64,
    #[serde(rename = "KALIIPBJGEB")]
    pub kaliipbjgeb: i64,
    #[serde(rename = "rewardID")]
    pub reward_id: i64,
}
