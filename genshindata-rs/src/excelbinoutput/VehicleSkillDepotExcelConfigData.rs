// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type VehicleSkillDepotExcelConfigData = Vec<VehicleSkillDepotExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleSkillDepotExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "JILDLKOLCID")]
    pub jildlkolcid: Vec<i64>,
}

pub fn load() -> Result<VehicleSkillDepotExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "VehicleSkillDepotExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
