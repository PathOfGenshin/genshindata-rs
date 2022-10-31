// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type BattlePassStoryExcelConfigData = Vec<BattlePassStoryExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BattlePassStoryExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "storyUnlockLevel")]
    pub story_unlock_level: Vec<i64>,

    #[serde(rename = "storyId")]
    pub story_id: Vec<i64>,

    #[serde(rename = "storyTitle")]
    pub story_title: Vec<i64>,
}

pub fn load() -> Result<BattlePassStoryExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "BattlePassStoryExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}