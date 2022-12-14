// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ReliquaryCodexExcelConfigData = Vec<ReliquaryCodexExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReliquaryCodexExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "suitId")]
    pub suit_id: i64,

    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "cupId")]
    pub cup_id: Option<i64>,

    #[serde(rename = "leatherId")]
    pub leather_id: Option<i64>,

    #[serde(rename = "capId")]
    pub cap_id: i64,

    #[serde(rename = "flowerId")]
    pub flower_id: Option<i64>,

    #[serde(rename = "sandId")]
    pub sand_id: Option<i64>,

    #[serde(rename = "SortOrder")]
    pub sort_order: i64,
}

pub fn load() -> Result<ReliquaryCodexExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ReliquaryCodexExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
