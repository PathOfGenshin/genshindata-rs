/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type RainbowPrinceSearchLevelExcelConfigData = Vec<RainbowPrinceSearchLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RainbowPrinceSearchLevelExcelConfigDatum {
    #[serde(rename = "galleryID")]
    pub gallery_id: i64,
    pub fikbhdchpla: i64,
    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,
    pub cmciegjjfok: i64,
    pub efilbifjgnc: i64,
    #[serde(rename = "levelID")]
    pub level_id: i64,
    #[serde(rename = "activityID")]
    pub activity_id: i64,
    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,
    #[serde(rename = "questList")]
    pub quest_list: Vec<i64>,
    pub hbdofodpcfo: Vec<Option<serde_json::Value>>,
    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,
    #[serde(rename = "openDay")]
    pub open_day: Option<i64>,
    pub kjballpjmej: Vec<String>,
    pub pkbhgfbaicn: Vec<String>,
    pub ikfkhdjonhh: Vec<String>,
    pub hpbgoogmani: Option<bool>,
}