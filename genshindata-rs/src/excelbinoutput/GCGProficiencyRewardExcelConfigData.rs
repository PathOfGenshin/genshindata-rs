// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GcgProficiencyRewardExcelConfigData = Vec<GcgProficiencyRewardExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GcgProficiencyRewardExcelConfigDatum {
    #[serde(rename = "KFDMIAPDCEG")]
    pub kfdmiapdceg: i64,

    #[serde(rename = "FJHMBDEAADC")]
    pub fjhmbdeaadc: Vec<Fjhmbdeaadc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fjhmbdeaadc {
    #[serde(rename = "JLFLKLLCBCJ")]
    pub jlflkllcbcj: Option<i64>,

    #[serde(rename = "rewardId")]
    pub reward_id: Option<i64>,
}

pub fn load() -> Result<GcgProficiencyRewardExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GCGProficiencyRewardExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
