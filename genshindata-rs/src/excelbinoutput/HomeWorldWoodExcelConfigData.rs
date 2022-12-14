// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type HomeWorldWoodExcelConfigData = Vec<HomeWorldWoodExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeWorldWoodExcelConfigDatum {
    #[serde(rename = "FAJJDLAFAAJ")]
    pub fajjdlafaaj: i64,

    #[serde(rename = "JMDENJBIFHK")]
    pub jmdenjbifhk: i64,

    #[serde(rename = "BPFOJNMPBDI")]
    pub bpfojnmpbdi: Vec<Vec<i64>>,
}

pub fn load() -> Result<HomeWorldWoodExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "HomeWorldWoodExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
