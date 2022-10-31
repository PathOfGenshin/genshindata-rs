// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type MoonfinTrialLevelExcelConfigData = Vec<MoonfinTrialLevelExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MoonfinTrialLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "LFHBNHPDLHD")]
    pub lfhbnhpdlhd: Vec<i64>,

    #[serde(rename = "mainQuest")]
    pub main_quest: Option<i64>,

    #[serde(rename = "ICOADJELPBO")]
    pub icoadjelpbo: Vec<f64>,

    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,

    #[serde(rename = "descriptionTextMapHash")]
    pub description_text_map_hash: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "AHNJPLCJGMP")]
    pub ahnjplcjgmp: Option<String>,

    #[serde(rename = "FBAEGDBCHBN")]
    pub fbaegdbchbn: Option<i64>,

    #[serde(rename = "galleryId")]
    pub gallery_id: Option<i64>,

    #[serde(rename = "challengeId")]
    pub challenge_id: Option<i64>,

    #[serde(rename = "HJNNGKNEFNA")]
    pub hjnngknefna: Option<i64>,

    #[serde(rename = "GFONAHPKIDO")]
    pub gfonahpkido: Option<i64>,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: Option<i64>,
}

pub fn load() -> Result<MoonfinTrialLevelExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "MoonfinTrialLevelExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}