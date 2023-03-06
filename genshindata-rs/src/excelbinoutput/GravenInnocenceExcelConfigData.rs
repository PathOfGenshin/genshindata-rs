// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GravenInnocenceExcelConfigData = Vec<GravenInnocenceExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GravenInnocenceExcelConfigDatum {
    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "DIIOHMENAAG")]
    pub diiohmenaag: i64,

    #[serde(rename = "BCOCGECKBAP")]
    pub bcocgeckbap: i64,

    #[serde(rename = "GHGLLNOIINO")]
    pub ghgllnoiino: i64,

    #[serde(rename = "MOMOFMOFDFB")]
    pub momofmofdfb: i64,

    #[serde(rename = "KEBFCAKPJPP")]
    pub kebfcakpjpp: i64,

    #[serde(rename = "FIMNLLGGEKK")]
    pub fimnllggekk: i64,

    #[serde(rename = "GHIPECIGOAM")]
    pub ghipecigoam: i64,

    #[serde(rename = "DONFDBGAHPB")]
    pub donfdbgahpb: i64,

    #[serde(rename = "ECJANAPEAEH")]
    pub ecjanapeaeh: i64,

    #[serde(rename = "DPHPMNOMGJA")]
    pub dphpmnomgja: Vec<i64>,

    #[serde(rename = "HEGANHNGKCD")]
    pub heganhngkcd: i64,

    #[serde(rename = "DKDHHLIMHNN")]
    pub dkdhhlimhnn: i64,
}

pub fn load() -> Result<GravenInnocenceExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GravenInnocenceExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
