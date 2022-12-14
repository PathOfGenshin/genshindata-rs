// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ElementCoeffExcelConfigData = Vec<ElementCoeffExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementCoeffExcelConfigDatum {
    #[serde(rename = "crashCo")]
    pub crash_co: f64,

    #[serde(rename = "elementLevelCo")]
    pub element_level_co: f64,

    #[serde(rename = "playerElementLevelCo")]
    pub player_element_level_co: f64,

    #[serde(rename = "playerShieldLevelCo")]
    pub player_shield_level_co: f64,

    #[serde(rename = "level")]
    pub level: Option<i64>,
}

pub fn load() -> Result<ElementCoeffExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ElementCoeffExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
