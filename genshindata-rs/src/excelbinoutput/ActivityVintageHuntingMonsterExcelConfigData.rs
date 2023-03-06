// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityVintageHuntingMonsterExcelConfigData = Vec<ActivityVintageHuntingMonsterExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityVintageHuntingMonsterExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "CNCOFPHBBHF")]
    pub cncofphbbhf: i64,

    #[serde(rename = "monsterIdList")]
    pub monster_id_list: Vec<i64>,
}

pub fn load() -> Result<ActivityVintageHuntingMonsterExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityVintageHuntingMonsterExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
