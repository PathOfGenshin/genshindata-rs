// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MonsterAffixExcelConfigData = Vec<MonsterAffixExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterAffixExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "affix")]
    pub affix: String,

    #[serde(rename = "comment")]
    pub comment: String,

    #[serde(rename = "abilityName")]
    pub ability_name: Vec<String>,

    #[serde(rename = "isLegal")]
    pub is_legal: IsLegal,

    #[serde(rename = "iconPath")]
    pub icon_path: IconPath,

    #[serde(rename = "generalSkillIcon")]
    pub general_skill_icon: String,

    #[serde(rename = "isCommon")]
    pub is_common: Option<bool>,

    #[serde(rename = "preAdd")]
    pub pre_add: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IconPath {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "Skill_E_Ambor")]
    SkillEAmbor,

    #[serde(rename = "Skill_E_Ayaka")]
    SkillEAyaka,

    #[serde(rename = "Skill_E_Diluc_01")]
    SkillEDiluc01,

    #[serde(rename = "Skill_E_Kate")]
    SkillEKate,

    #[serde(rename = "Skill_E_PlayerWind_01")]
    SkillEPlayerWind01,

    #[serde(rename = "Skill_S_Diluc_01_01")]
    SkillSDiluc01_01,

    #[serde(rename = "Skill_S_Diluc_01_02")]
    SkillSDiluc01_02,

    #[serde(rename = "Skill_S_Diluc_01_03")]
    SkillSDiluc01_03,

    #[serde(rename = "Skill_S_Qin_01")]
    SkillSQin01,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IsLegal {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "是")]
    IsLegal,
}

pub fn load() -> Result<MonsterAffixExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MonsterAffixExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
