// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BrickBreakerSkillExcelConfigData = Vec<BrickBreakerSkillExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BrickBreakerSkillExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "maxLevel")]
    pub max_level: i64,

    #[serde(rename = "HCNIILDMFNM")]
    pub hcniildmfnm: i64,

    #[serde(rename = "IFLGLLEHDNL")]
    pub iflgllehdnl: i64,

    #[serde(rename = "descParam")]
    pub desc_param: Vec<String>,

    #[serde(rename = "HBPNPFPCLGH")]
    pub hbpnpfpclgh: i64,

    #[serde(rename = "unlockWorldLevel")]
    pub unlock_world_level: Option<i64>,

    #[serde(rename = "PACDNBGOPAB")]
    pub pacdnbgopab: Vec<i64>,

    #[serde(rename = "ABPEKGBELEK")]
    pub abpekgbelek: String,

    #[serde(rename = "HKDDDPECLDG")]
    pub hkdddpecldg: Option<i64>,
}

pub fn load() -> Result<BrickBreakerSkillExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "BrickBreakerSkillExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
