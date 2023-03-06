// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AbilityOverrideExcelConfigData = Vec<AbilityOverrideExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AbilityOverrideExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "abilityName")]
    pub ability_name: String,

    #[serde(rename = "LDACIJGPBFK")]
    pub ldacijgpbfk: Vec<Ldacijgpbfk>,

    #[serde(rename = "AOKCEFFGCNH")]
    pub aokceffgcnh: Vec<String>,

    #[serde(rename = "DEGOHICJMNL")]
    pub degohicjmnl: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ldacijgpbfk {
    #[serde(rename = "DGOEFEALPHG")]
    pub dgoefealphg: String,

    #[serde(rename = "LHKECFAMKEI")]
    pub lhkecfamkei: Option<f64>,
}

pub fn load() -> Result<AbilityOverrideExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "AbilityOverrideExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
