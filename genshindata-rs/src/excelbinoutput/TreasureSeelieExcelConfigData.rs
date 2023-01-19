// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type TreasureSeelieExcelConfigData = Vec<TreasureSeelieExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TreasureSeelieExcelConfigDatum {
    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "IAFNGOPOCAK")]
    pub iafngopocak: i64,

    #[serde(rename = "BMKENNHNKME")]
    pub bmkennhnkme: i64,

    #[serde(rename = "FJIDNGOMMKI")]
    pub fjidngommki: i64,
}

pub fn load() -> Result<TreasureSeelieExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "TreasureSeelieExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
