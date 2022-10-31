// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type AchievementGoalExcelConfigData = Vec<AchievementGoalExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementGoalExcelConfigDatum {
    #[serde(rename = "orderId")]
    pub order_id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "iconPath")]
    pub icon_path: String,

    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "finishRewardId")]
    pub finish_reward_id: Option<i64>,
}

pub fn load() -> Result<AchievementGoalExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "AchievementGoalExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}