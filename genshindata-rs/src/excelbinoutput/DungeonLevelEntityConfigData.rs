// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type DungeonLevelEntityConfigData = Vec<DungeonLevelEntityConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DungeonLevelEntityConfigDatum {
    #[serde(rename = "clientId")]
    pub client_id: i64,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "show")]
    pub show: Option<bool>,

    #[serde(rename = "levelConfigName")]
    pub level_config_name: String,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "PCGMJGFJHJI")]
    pub pcgmjgfjhji: i64,
}

pub fn load() -> Result<DungeonLevelEntityConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "DungeonLevelEntityConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
