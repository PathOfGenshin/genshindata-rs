/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityRockBoardExploreQuestExcelConfigData = Vec<ActivityRockBoardExploreQuestExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ActivityRockBoardExploreQuestExcelConfigDatum {
    pub fhnmenjgfgb: i64,
    #[serde(rename = "questID")]
    pub quest_id: i64,
    #[serde(rename = "openDay")]
    pub open_day: i64,
    #[serde(rename = "iconName")]
    pub icon_name: String,
    pub eeeaioeaedc: i64,
    pub ijcehgbanbf: i64,
}
