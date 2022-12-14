// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type TreasureSeelieRegionExcelConfigData = Vec<TreasureSeelieRegionExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TreasureSeelieRegionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "EDNGLANAEMJ")]
    pub ednglanaemj: i64,

    #[serde(rename = "KMPJKBEAFKD")]
    pub kmpjkbeafkd: Vec<i64>,

    #[serde(rename = "ANFMMOIKMAF")]
    pub anfmmoikmaf: i64,
}

pub fn load() -> Result<TreasureSeelieRegionExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "TreasureSeelieRegionExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
