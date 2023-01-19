// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type TeamChainBuffExcelConfigData = Vec<TeamChainBuffExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamChainBuffExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "BHCJKANDNPF")]
    pub bhcjkandnpf: String,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "buffNameTextMapHash")]
    pub buff_name_text_map_hash: i64,

    #[serde(rename = "DNMPGBBMMPI")]
    pub dnmpgbbmmpi: i64,

    #[serde(rename = "LPIFKEDCMND")]
    pub lpifkedcmnd: Vec<String>,
}

pub fn load() -> Result<TeamChainBuffExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "TeamChainBuffExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}