// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type CoopExcelConfigData = Vec<CoopExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CoopExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "CoopCfg")]
    pub coop_cfg: String,
}

pub fn load() -> Result<CoopExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "CoopExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
