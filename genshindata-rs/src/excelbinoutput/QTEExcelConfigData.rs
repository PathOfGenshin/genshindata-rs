// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type QteExcelConfigData = Vec<QteExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct QteExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "OMKAMGFAEEE")]
    pub omkamgfaeee: i64,

    #[serde(rename = "MAIIKGNPIEL")]
    pub maiikgnpiel: String,

    #[serde(rename = "JLMKFMIDAEG")]
    pub jlmkfmidaeg: Vec<Jlmkfmidaeg>,

    #[serde(rename = "EKGDKOLPOCH")]
    pub ekgdkolpoch: Vec<Ekgdkolpoch>,

    #[serde(rename = "PMFICKDBLFJ")]
    pub pmfickdblfj: Vec<Jlmkfmidaeg>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ekgdkolpoch {
    #[serde(rename = "type")]
    pub ekgdkolpoch_type: Option<String>,

    #[serde(rename = "param")]
    pub param: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Jlmkfmidaeg {
    #[serde(rename = "param")]
    pub param: Vec<i64>,
}

pub fn load() -> Result<QteExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "QTEExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
