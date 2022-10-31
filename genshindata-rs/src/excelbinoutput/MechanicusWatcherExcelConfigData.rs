// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type MechanicusWatcherExcelConfigData = Vec<MechanicusWatcherExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MechanicusWatcherExcelConfigDatum {
    #[serde(rename = "rewardTokens")]
    pub reward_tokens: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "triggerConfig")]
    pub trigger_config: TriggerConfig,

    #[serde(rename = "progress")]
    pub progress: i64,

    #[serde(rename = "isDisuse")]
    pub is_disuse: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerConfig {
    #[serde(rename = "triggerType")]
    pub trigger_type: String,

    #[serde(rename = "paramList")]
    pub param_list: Vec<String>,
}

pub fn load() -> Result<MechanicusWatcherExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "MechanicusWatcherExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}