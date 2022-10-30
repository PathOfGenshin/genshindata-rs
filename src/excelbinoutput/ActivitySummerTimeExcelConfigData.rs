// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ActivitySummerTimeExcelConfigData = Vec<ActivitySummerTimeExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivitySummerTimeExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "unlockQuestID")]
    pub unlock_quest_id: i64,

    #[serde(rename = "contentDuration")]
    pub content_duration: i64,

    #[serde(rename = "unlockPlayerLevel")]
    pub unlock_player_level: i64,

    #[serde(rename = "FIPDBBFPGMD")]
    pub fipdbbfpgmd: i64,

    #[serde(rename = "CHLAMMFJKAP")]
    pub chlammfjkap: Vec<i64>,

    #[serde(rename = "rewardPreview")]
    pub reward_preview: i64,

    #[serde(rename = "questIdList")]
    pub quest_id_list: Vec<i64>,
}

pub fn load() -> Result<ActivitySummerTimeExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ActivitySummerTimeExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
