// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ExpeditionActivityMarkerExcelConfigData = Vec<ExpeditionActivityMarkerExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpeditionActivityMarkerExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "posX")]
    pub pos_x: f64,

    #[serde(rename = "posY")]
    pub pos_y: f64,

    #[serde(rename = "pictureHash")]
    pub picture_hash: i64,
}

pub fn load() -> Result<ExpeditionActivityMarkerExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ExpeditionActivityMarkerExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
