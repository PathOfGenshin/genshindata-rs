/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FungusFighterV3ExcelConfigData = Vec<FungusFighterV3ExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FungusFighterV3ExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,
    #[serde(rename = "dayIndex")]
    pub day_index: i64,
    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,
    #[serde(rename = "galleryId")]
    pub gallery_id: i64,
    pub lmglahhjfkn: Vec<i64>,
    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,
    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,
    pub cbdbkhlfldo: i64,
    pub logcfijkfcg: i64,
    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,
    pub nibghhjeboh: Option<i64>,
    pub hjkbodemgam: Option<bool>,
    pub pknnbpjcnma: Vec<i64>,
    pub aklkgfnpmne: Vec<i64>,
    pub lodpdmdcffp: Vec<i64>,
    pub hbidbopenhf: String,
    pub lelgbgpgbal: String,
    pub mkhcbkikfjb: i64,
    pub nmelfbbhgoi: i64,
    pub maoklamadhe: Option<i64>,
    pub lgoibiibhhg: Option<bool>,
    #[serde(rename = "preQuestId")]
    pub pre_quest_id: Option<i64>,
    pub oipfongbeeh: Option<i64>,
}