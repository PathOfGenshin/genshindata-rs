// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type FleurFairDungeonStatExcelConfigData = Vec<FleurFairDungeonStatExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FleurFairDungeonStatExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: Option<i64>,

    #[serde(rename = "statType")]
    pub stat_type: StatType,

    #[serde(rename = "statParamList")]
    pub stat_param_list: Vec<String>,

    #[serde(rename = "orderingType")]
    pub ordering_type: OrderingType,

    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "statMethod")]
    pub stat_method: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderingType {
    #[serde(rename = "Equal")]
    Equal,

    #[serde(rename = "Greater")]
    Greater,

    #[serde(rename = "LessOrEqual")]
    LessOrEqual,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StatType {
    #[serde(rename = "FLEUR_FAIR_DUNGEON_STAT_GROUP_VARIABLE")]
    FleurFairDungeonStatGroupVariable,

    #[serde(rename = "FLEUR_FAIR_DUNGEON_STAT_MONSTER_HURT")]
    FleurFairDungeonStatMonsterHurt,
}

pub fn load() -> Result<FleurFairDungeonStatExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "FleurFairDungeonStatExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
