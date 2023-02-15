// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LanV3RaceExcelConfigData = Vec<LanV3RaceExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LanV3RaceExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "JKAMKKGPBBJ")]
    pub jkamkkgpbbj: i64,

    #[serde(rename = "CFCKMLPNMCP")]
    pub cfckmlpnmcp: Vec<i64>,

    #[serde(rename = "DMLBFPHOADL")]
    pub dmlbfphoadl: f64,

    #[serde(rename = "LJODAJCOPGL")]
    pub ljodajcopgl: f64,

    #[serde(rename = "EIPIEAFPDFN")]
    pub eipieafpdfn: Vec<i64>,

    #[serde(rename = "ODIOKGBEMLP")]
    pub odiokgbemlp: f64,

    #[serde(rename = "FDKDMFNBPFH")]
    pub fdkdmfnbpfh: i64,

    #[serde(rename = "CILFJBMLBFH")]
    pub cilfjbmlbfh: f64,
}

pub fn load() -> Result<LanV3RaceExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "LanV3RaceExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
