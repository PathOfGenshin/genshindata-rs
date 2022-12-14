// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type SpriteTagExcelConfigData = Vec<SpriteTagExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteTagExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "GDNCEIONHEF")]
    pub gdnceionhef: String,

    #[serde(rename = "DHKJGCENHKK")]
    pub dhkjgcenhkk: f64,

    #[serde(rename = "GPKIPJFFMJA")]
    pub gpkipjffmja: i64,
}

pub fn load() -> Result<SpriteTagExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "SpriteTagExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
