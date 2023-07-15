/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type WinterCampExploreExcelConfigData = Vec<WinterCampExploreExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WinterCampExploreExcelConfigDatum {
    pub id: i64,
    pub open_day: i64,
    pub priority: i64,
    #[serde(rename = "groupID")]
    pub group_id: i64,
    #[serde(rename = "PGGIJCDNCAM")]
    pub pggijcdncam: i64,
    #[serde(rename = "rewardID")]
    pub reward_id: i64,
    #[serde(rename = "EALHMLABOPA")]
    pub ealhmlabopa: Vec<Ealhmlabopa>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Ealhmlabopa {
    pub geddcdajgdo: Vec<i64>,
}
