// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type PsActivitiesActivityConfigData = Vec<PsActivitiesActivityConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PsActivitiesActivityConfigDatum {
    #[serde(rename = "activityNameTextMapHash")]
    pub activity_name_text_map_hash: i64,

    #[serde(rename = "activityDescTextMapHash")]
    pub activity_desc_text_map_hash: i64,

    #[serde(rename = "availableByDefault")]
    pub available_by_default: Option<bool>,

    #[serde(rename = "isRequiredForCompletion")]
    pub is_required_for_completion: bool,

    #[serde(rename = "largeIcon")]
    pub large_icon: String,

    #[serde(rename = "smallIcon")]
    pub small_icon: String,

    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "objectID")]
    pub object_id: String,

    #[serde(rename = "IBMHEFDMDJC")]
    pub ibmhefdmdjc: String,
}

pub fn load() -> Result<PsActivitiesActivityConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "PSActivitiesActivityConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
