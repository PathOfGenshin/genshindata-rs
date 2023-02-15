// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgScenePointExcelConfigData = Vec<GcgScenePointExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GcgScenePointExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "pos")]
    pub pos: Vec<f64>,

    #[serde(rename = "OELGFMCEKAC")]
    pub oelgfmcekac: f64,

    #[serde(rename = "INCNPFFFHIJ")]
    pub incnpfffhij: Option<Incnpfffhij>,

    #[serde(rename = "NNEMGINJCFC")]
    pub nnemginjcfc: Vec<Nnemginjcfc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Incnpfffhij {
    #[serde(rename = "POINT_MOTION_SIT")]
    PointMotionSit,

    #[serde(rename = "POINT_MOTION_STAND")]
    PointMotionStand,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Nnemginjcfc {
    #[serde(rename = "SUPPORT_CALL")]
    SupportCall,

    #[serde(rename = "SUPPORT_NONE")]
    SupportNone,
}

pub fn load() -> Result<GcgScenePointExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GCGScenePointExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
