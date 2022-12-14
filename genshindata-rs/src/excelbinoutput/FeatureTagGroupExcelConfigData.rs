// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type FeatureTagGroupExcelConfigData = Vec<FeatureTagGroupExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureTagGroupExcelConfigDatum {
    #[serde(rename = "groupID")]
    pub group_id: i64,

    #[serde(rename = "tagIDs")]
    pub tag_i_ds: Vec<i64>,
}

pub fn load() -> Result<FeatureTagGroupExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "FeatureTagGroupExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
