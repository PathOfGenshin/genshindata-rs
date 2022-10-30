// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ActivityArenaChallengePreviewExcelConfigData = Vec<ActivityArenaChallengePreviewExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityArenaChallengePreviewExcelConfigDatum {
    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "centerPosList")]
    pub center_pos_list: Vec<f64>,

    #[serde(rename = "guideQuestId1")]
    pub guide_quest_id1: i64,

    #[serde(rename = "OPIFMHAGBPD")]
    pub opifmhagbpd: Option<i64>,

    #[serde(rename = "NBDHGAKNLPM")]
    pub nbdhgaknlpm: Option<i64>,
}

pub fn load() -> Result<ActivityArenaChallengePreviewExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ActivityArenaChallengePreviewExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
