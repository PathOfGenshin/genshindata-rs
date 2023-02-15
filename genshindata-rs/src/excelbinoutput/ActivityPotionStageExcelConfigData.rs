// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityPotionStageExcelConfigData = Vec<ActivityPotionStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityPotionStageExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "KHAKJEOMIAC")]
    pub khakjeomiac: Vec<i64>,

    #[serde(rename = "GAHONNOEJFP")]
    pub gahonnoejfp: i64,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "OBAOGFIILAL")]
    pub obaogfiilal: Vec<i64>,

    #[serde(rename = "HGHDONCFIMD")]
    pub hghdoncfimd: Vec<i64>,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "AJJFHOEKKCC")]
    pub ajjfhoekkcc: i64,
}

pub fn load() -> Result<ActivityPotionStageExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityPotionStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
