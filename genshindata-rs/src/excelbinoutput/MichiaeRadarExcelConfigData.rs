// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MichiaeRadarExcelConfigData = Vec<MichiaeRadarExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MichiaeRadarExcelConfigDatum {
    #[serde(rename = "FKCOMOMCLCB")]
    pub fkcomomclcb: String,

    #[serde(rename = "FIBKNGALEJO")]
    pub fibkngalejo: Vec<i64>,
}

pub fn load() -> Result<MichiaeRadarExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MichiaeRadarExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
