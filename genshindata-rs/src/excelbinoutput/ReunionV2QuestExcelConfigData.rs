/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ReunionV2QuestExcelConfigData = Vec<ReunionV2QuestExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReunionV2QuestExcelConfigDatum {
    pub id: i64,
    pub priority: i64,
    #[serde(rename = "LNOIGBLCFMA")]
    pub lnoigblcfma: String,
    #[serde(rename = "APPKOCMCFJJ")]
    pub appkocmcfjj: Appkocmcfjj,
    #[serde(rename = "BCPOLEBAIBD")]
    pub bcpolebaibd: String,
    #[serde(rename = "OIDEHFHBBFJ")]
    pub oidehfhbbfj: String,
    pub watcher_list: Vec<i64>,
    #[serde(rename = "DGDBHDDDEKJ")]
    pub dgdbhdddekj: Vec<i64>,
    pub tab_name_text_map_hash: i64,
    #[serde(rename = "FLLLIDOEJOC")]
    pub flllidoejoc: i64,
    pub chapter_id: i64,
    pub desc_text_map_hash: i64,
    pub required_player_level: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Appkocmcfjj {
    #[serde(rename = "ART/Effect/UI/UI/Eff_UI_ReunionImg_KV_01")]
    ArtEffectUiUiEffUiReunionImgKv01,
}
