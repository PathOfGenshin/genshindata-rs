// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type RandomCompoundDisplayExcelConfigData = Vec<RandomCompoundDisplayExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomCompoundDisplayExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "KKJGPFNCGFA")]
    pub kkjgpfncgfa: Vec<Kkjgpfncgfa>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kkjgpfncgfa {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

pub fn load() -> Result<RandomCompoundDisplayExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "RandomCompoundDisplayExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}