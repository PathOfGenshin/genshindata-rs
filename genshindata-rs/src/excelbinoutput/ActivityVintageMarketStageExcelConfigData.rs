/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityVintageMarketStageExcelConfigData = Vec<ActivityVintageMarketStageExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ActivityVintageMarketStageExcelConfigDatum {
    #[serde(rename = "stageID")]
    pub stage_id: i64,
    pub loihopdmnei: i64,
    #[serde(rename = "openDay")]
    pub open_day: i64,
    #[serde(rename = "openQuestID")]
    pub open_quest_id: i64,
    pub ekbbgcagkga: i64,
    pub dpafhbdclmn: i64,
    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,
    #[serde(rename = "tutorialID")]
    pub tutorial_id: i64,
    pub fgbjephmake: i64,
    pub algbgpliplb: Option<i64>,
}
