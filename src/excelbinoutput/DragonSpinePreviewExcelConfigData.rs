// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type DragonSpinePreviewExcelConfigData = Vec<DragonSpinePreviewExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DragonSpinePreviewExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "preQuestId")]
    pub pre_quest_id: i64,

    #[serde(rename = "unlockLevel")]
    pub unlock_level: i64,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "contentDuration")]
    pub content_duration: i64,

    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "questIdList")]
    pub quest_id_list: Vec<i64>,
}

pub fn load() -> Result<DragonSpinePreviewExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "DragonSpinePreviewExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
