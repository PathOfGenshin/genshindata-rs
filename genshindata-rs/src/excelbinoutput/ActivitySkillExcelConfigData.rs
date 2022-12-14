// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivitySkillExcelConfigData = Vec<ActivitySkillExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivitySkillExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "skillTarget")]
    pub skill_target: Option<SkillTarget>,

    #[serde(rename = "abilityName")]
    pub ability_name: String,

    #[serde(rename = "globalValueKey")]
    pub global_value_key: String,

    #[serde(rename = "energyMin")]
    pub energy_min: Option<i64>,

    #[serde(rename = "energyMax")]
    pub energy_max: i64,

    #[serde(rename = "EEOPHAJKBJB")]
    pub eeophajkbjb: Vec<i64>,

    #[serde(rename = "cdTime")]
    pub cd_time: f64,

    #[serde(rename = "guideTime")]
    pub guide_time: Option<f64>,

    #[serde(rename = "skillIcon")]
    pub skill_icon: String,

    #[serde(rename = "guideKey")]
    pub guide_key: Vec<String>,

    #[serde(rename = "guideOpenState")]
    pub guide_open_state: Option<String>,

    #[serde(rename = "unableTextTextMapHash")]
    pub unable_text_text_map_hash: i64,

    #[serde(rename = "channelTextTextMapHash")]
    pub channel_text_text_map_hash: i64,

    #[serde(rename = "interruptTextTextMapHash")]
    pub interrupt_text_text_map_hash: i64,

    #[serde(rename = "JGPJNNADHEA")]
    pub jgpjnnadhea: Option<i64>,

    #[serde(rename = "NGNOHCDNOKH")]
    pub ngnohcdnokh: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SkillTarget {
    #[serde(rename = "AST_PLAY")]
    AstPlay,

    #[serde(rename = "AST_TEAM")]
    AstTeam,
}

pub fn load() -> Result<ActivitySkillExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivitySkillExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
