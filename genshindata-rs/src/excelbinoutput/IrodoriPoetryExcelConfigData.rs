/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type IrodoriPoetryExcelConfigData = Vec<IrodoriPoetryExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IrodoriPoetryExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "condID")]
    pub cond_id: i64,
    #[serde(rename = "themeCondID")]
    pub theme_cond_id: i64,
    pub unlock_day: i64,
    pub entity_type: String,
    pub scan_config_list: Vec<ScanConfigList>,
    #[serde(rename = "mainQuestID")]
    pub main_quest_id: i64,
    #[serde(rename = "minInspirationQuestID")]
    pub min_inspiration_quest_id: i64,
    #[serde(rename = "fillPoetryQuestID")]
    pub fill_poetry_quest_id: i64,
    #[serde(rename = "reminderIDList")]
    pub reminder_id_list: Vec<i64>,
    #[serde(rename = "watcherID")]
    pub watcher_id: i64,
    pub camera_hint_text_map_hash: i64,
    pub theme_title_text_map_hash: i64,
    pub theme_desc_text_map_hash: i64,
    pub poetry_title_text_map_hash: i64,
    #[serde(rename = "existsLineIDList")]
    pub exists_line_id_list: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanConfigList {
    pub index_id_list: Vec<i64>,
    pub line_id: Option<i64>,
}
