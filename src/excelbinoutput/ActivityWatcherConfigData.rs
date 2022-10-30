// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ActivityWatcherConfigData = Vec<ActivityWatcherConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityWatcherConfigDatum {
    #[serde(rename = "RewardId")]
    pub reward_id: i64,

    #[serde(rename = "RewardPreview")]
    pub reward_preview: i64,

    #[serde(rename = "ActivitychallengetipsTextMapHash")]
    pub activitychallengetips_text_map_hash: i64,

    #[serde(rename = "ExtraActivitychallengetipsTextMapHash")]
    pub extra_activitychallengetips_text_map_hash: i64,

    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "TriggerConfig")]
    pub trigger_config: TriggerConfig,

    #[serde(rename = "Progress")]
    pub progress: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerConfig {
    #[serde(rename = "TriggerType")]
    pub trigger_type: String,

    #[serde(rename = "Param")]
    pub param: Vec<String>,
}

pub fn load() -> Result<ActivityWatcherConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ActivityWatcherConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
