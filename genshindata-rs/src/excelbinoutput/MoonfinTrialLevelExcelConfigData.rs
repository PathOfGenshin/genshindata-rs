// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type MoonfinTrialLevelExcelConfigData = Vec<MoonfinTrialLevelExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MoonfinTrialLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "MNCMLAMFHHJ")]
    pub mncmlamfhhj: Vec<i64>,

    #[serde(rename = "mainQuest")]
    pub main_quest: Option<i64>,

    #[serde(rename = "JBKKLHNDHEP")]
    pub jbkklhndhep: Vec<f64>,

    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,

    #[serde(rename = "descriptionTextMapHash")]
    pub description_text_map_hash: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "EFEOOJFNKLN")]
    pub efeoojfnkln: Option<String>,

    #[serde(rename = "LDNAFOJBJLK")]
    pub ldnafojbjlk: Option<i64>,

    #[serde(rename = "galleryId")]
    pub gallery_id: Option<i64>,

    #[serde(rename = "challengeId")]
    pub challenge_id: Option<i64>,

    #[serde(rename = "KCHAGIJLMEH")]
    pub kchagijlmeh: Option<i64>,

    #[serde(rename = "BDJJOHAKCEL")]
    pub bdjjohakcel: Option<i64>,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: Option<i64>,
}

pub fn load() -> Result<MoonfinTrialLevelExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MoonfinTrialLevelExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
