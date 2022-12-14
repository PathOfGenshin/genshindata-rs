// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type MechanicusMapPointExcelConfigData = Vec<MechanicusMapPointExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MechanicusMapPointExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "coordX")]
    pub coord_x: f64,

    #[serde(rename = "coordY")]
    pub coord_y: f64,
}

pub fn load() -> Result<MechanicusMapPointExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MechanicusMapPointExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
