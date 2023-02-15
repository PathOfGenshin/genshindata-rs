// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FungusTrainingDungeonEnemyAffixExcelConfigData = Vec<FungusTrainingDungeonEnemyAffixExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FungusTrainingDungeonEnemyAffixExcelConfigDatum {
    #[serde(rename = "affixId")]
    pub affix_id: i64,

    #[serde(rename = "JDPCOEKBOFI")]
    pub jdpcoekbofi: i64,
}

pub fn load() -> Result<FungusTrainingDungeonEnemyAffixExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "FungusTrainingDungeonEnemyAffixExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
