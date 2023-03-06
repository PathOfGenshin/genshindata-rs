// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type InstableSprayBuffExcelConfigData = Vec<InstableSprayBuffExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct InstableSprayBuffExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "LADKLKNNEKD")]
    pub ladklknnekd: String,

    #[serde(rename = "EJPOCJICPGN")]
    pub ejpocjicpgn: String,

    #[serde(rename = "KKHHICCNGGI")]
    pub kkhhiccnggi: i64,

    #[serde(rename = "elementType")]
    pub element_type: i64,

    #[serde(rename = "buffNameTextMapHash")]
    pub buff_name_text_map_hash: i64,

    #[serde(rename = "EOPGNNKMNGO")]
    pub eopgnnkmngo: i64,

    #[serde(rename = "NBDPLIKACFG")]
    pub nbdplikacfg: Vec<String>,
}

pub fn load() -> Result<InstableSprayBuffExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "InstableSprayBuffExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
