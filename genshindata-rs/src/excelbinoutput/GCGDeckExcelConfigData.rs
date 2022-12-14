// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GcgDeckExcelConfigData = Vec<GcgDeckExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GcgDeckExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "CBPFHHBCHFI")]
    pub cbpfhhbchfi: Vec<i64>,

    #[serde(rename = "NPHKAKLDHOC")]
    pub nphkakldhoc: Vec<i64>,

    #[serde(rename = "FOCNIOAIGEG")]
    pub focnioaigeg: Vec<Option<serde_json::Value>>,

    #[serde(rename = "NMKLIBEPKNO")]
    pub nmklibepkno: Vec<Option<serde_json::Value>>,

    #[serde(rename = "FEJOINMFEPP")]
    pub fejoinmfepp: Vec<Fejoinmfepp>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fejoinmfepp {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "KKPJFOJGEPB")]
    pub kkpjfojgepb: Option<i64>,
}

pub fn load() -> Result<GcgDeckExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GCGDeckExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
