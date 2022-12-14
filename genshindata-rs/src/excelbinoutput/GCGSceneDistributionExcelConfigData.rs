// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GcgSceneDistributionExcelConfigData = Vec<GcgSceneDistributionExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GcgSceneDistributionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "KJMMOCPNADC")]
    pub kjmmocpnadc: Vec<i64>,

    #[serde(rename = "CKFFBOEGINL")]
    pub ckffboeginl: Vec<i64>,

    #[serde(rename = "EMEILJJJLNE")]
    pub emeiljjjlne: i64,

    #[serde(rename = "EMNEKFFNINA")]
    pub emnekffnina: i64,
}

pub fn load() -> Result<GcgSceneDistributionExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GCGSceneDistributionExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
