// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GcgSceneConstNpcExcelConfigData = Vec<GcgSceneConstNpcExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GcgSceneConstNpcExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "npcId")]
    pub npc_id: i64,

    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "DGHFOKDOHMD")]
    pub dghfokdohmd: Vec<Option<serde_json::Value>>,
}

pub fn load() -> Result<GcgSceneConstNpcExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GCGSceneConstNpcExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
