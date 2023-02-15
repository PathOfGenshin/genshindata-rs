// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type HomeWorldServerGadgetExcelConfigData = Vec<HomeWorldServerGadgetExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeWorldServerGadgetExcelConfigDatum {
    #[serde(rename = "GOHAIPPNPCI")]
    pub gohaippnpci: i64,

    #[serde(rename = "IONADLOECKM")]
    pub ionadloeckm: i64,
}

pub fn load() -> Result<HomeWorldServerGadgetExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "HomeWorldServerGadgetExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
