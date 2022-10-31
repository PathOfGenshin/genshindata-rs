// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type TowerLevelExcelConfigData = Vec<TowerLevelExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TowerLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "levelGroupId")]
    pub level_group_id: i64,

    #[serde(rename = "levelIndex")]
    pub level_index: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "conds")]
    pub conds: Vec<Cond>,

    #[serde(rename = "towerBuffConfigStrList")]
    pub tower_buff_config_str_list: Vec<String>,

    #[serde(rename = "firstPassRewardId")]
    pub first_pass_reward_id: i64,

    #[serde(rename = "monsterLevel")]
    pub monster_level: i64,

    #[serde(rename = "firstMonsterList")]
    pub first_monster_list: Vec<i64>,

    #[serde(rename = "secondMonsterList")]
    pub second_monster_list: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cond {
    #[serde(rename = "towerCondType")]
    pub tower_cond_type: TowerCondType,

    #[serde(rename = "OECDJKGBNFE")]
    pub oecdjkgbnfe: Vec<i64>,

    #[serde(rename = "argumentList")]
    pub argument_list: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TowerCondType {
    #[serde(rename = "TOWER_COND_CHALLENGE_LEFT_TIME_MORE_THAN")]
    TowerCondChallengeLeftTimeMoreThan,

    #[serde(rename = "TOWER_COND_LEFT_HP_GREATER_THAN")]
    TowerCondLeftHpGreaterThan,
}

pub fn load() -> Result<TowerLevelExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "TowerLevelExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}