/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FlightGearLevelExcelConfigData = Vec<FlightGearLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FlightGearLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,
    #[serde(rename = "stage")]
    pub stage: i64,
    #[serde(rename = "galleryId")]
    pub gallery_id: i64,
    #[serde(rename = "groupId")]
    pub group_id: i64,
    #[serde(rename = "groupLinkId")]
    pub group_link_id: i64,
    pub ekbpajiilfk: i64,
    pub cankapnfagn: String,
    pub eipaokidmll: f64,
    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,
    pub gfeedohajah: i64,
    pub obpejflgbgk: i64,
    pub hhepeapehin: i64,
    pub fkjkkmkejfo: i64,
    #[serde(rename = "scoreLevelList")]
    pub score_level_list: Vec<i64>,
    pub gmcjifdjgoc: Option<i64>,
}
