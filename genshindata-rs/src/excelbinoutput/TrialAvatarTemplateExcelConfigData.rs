// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type TrialAvatarTemplateExcelConfigData = Vec<TrialAvatarTemplateExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TrialAvatarTemplateExcelConfigDatum {
    #[serde(rename = "TrialAvatarLevel")]
    pub trial_avatar_level: i64,

    #[serde(rename = "TrialReliquaryList")]
    pub trial_reliquary_list: Vec<i64>,

    #[serde(rename = "TrialTalentList")]
    pub trial_talent_list: Vec<Option<serde_json::Value>>,

    #[serde(rename = "TrialAvatarSkillLevel")]
    pub trial_avatar_skill_level: i64,
}

pub fn load() -> Result<TrialAvatarTemplateExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "TrialAvatarTemplateExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
