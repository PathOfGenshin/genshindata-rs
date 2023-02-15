// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type WorldExcelConfigData = Vec<WorldExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "type")]
    pub world_excel_config_datum_type: String,

    #[serde(rename = "mainSceneId")]
    pub main_scene_id: i64,

    #[serde(rename = "subSceneIdVec")]
    pub sub_scene_id_vec: Vec<i64>,
}

pub fn load() -> Result<WorldExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "WorldExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
