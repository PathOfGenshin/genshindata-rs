// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type NewActivityCondExcelConfigData = Vec<NewActivityCondExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewActivityCondExcelConfigDatum {
    #[serde(rename = "condId")]
    pub cond_id: i64,

    #[serde(rename = "condComb")]
    pub cond_comb: Option<CondComb>,

    #[serde(rename = "cond")]
    pub cond: Vec<Cond>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cond {
    #[serde(rename = "type")]
    pub cond_type: Option<String>,

    #[serde(rename = "param")]
    pub param: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CondComb {
    #[serde(rename = "LOGIC_AND")]
    LogicAnd,

    #[serde(rename = "LOGIC_OR")]
    LogicOr,
}

pub fn load() -> Result<NewActivityCondExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "NewActivityCondExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
