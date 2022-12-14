// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type CaptureExcelConfigData = Vec<CaptureExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CaptureExcelConfigDatum {
    #[serde(rename = "monsterID")]
    pub monster_id: i64,

    #[serde(rename = "GOOJHAFGOLF")]
    pub goojhafgolf: Vec<Goojhafgolf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Goojhafgolf {
    #[serde(rename = "MHJNKEOGOHM")]
    pub mhjnkeogohm: Option<i64>,

    #[serde(rename = "dropID")]
    pub drop_id: Option<i64>,

    #[serde(rename = "itemID")]
    pub item_id: Option<i64>,
}

pub fn load() -> Result<CaptureExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "CaptureExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
