// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type HuntingClueMonsterExcelConfigData = Vec<HuntingClueMonsterExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HuntingClueMonsterExcelConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "monsterId")]
    pub monster_id: i64,

    #[serde(rename = "reviseLevelId")]
    pub revise_level_id: i64,

    #[serde(rename = "groupType")]
    pub group_type: GroupType,

    #[serde(rename = "monsterGroupId")]
    pub monster_group_id: i64,

    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "isClueMonster")]
    pub is_clue_monster: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GroupType {
    #[serde(rename = "HUNTING_GROUP_TYPE_CLUE")]
    HuntingGroupTypeClue,
}

pub fn load() -> Result<HuntingClueMonsterExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "HuntingClueMonsterExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
