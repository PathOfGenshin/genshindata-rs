// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ExploreAreaTotalExpExcelConfigData = Vec<ExploreAreaTotalExpExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExploreAreaTotalExpExcelConfigDatum {
    #[serde(rename = "areaID")]
    pub area_id: i64,

    #[serde(rename = "totalExp")]
    pub total_exp: i64,

    #[serde(rename = "KMIOJKEFDOO")]
    pub kmiojkefdoo: Option<f64>,
}

pub fn load() -> Result<ExploreAreaTotalExpExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ExploreAreaTotalExpExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
