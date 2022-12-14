// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivityBannerExcelConfigData = Vec<ActivityBannerExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityBannerExcelConfigDatum {
    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "GBGPIHIHHAL")]
    pub gbgpihihhal: Option<Gbgpihihhal>,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: Option<i64>,

    #[serde(rename = "LECOGHPIOPO")]
    pub lecoghpiopo: String,

    #[serde(rename = "prefabPath")]
    pub prefab_path: String,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "OCJPKPOMGLB")]
    pub ocjpkpomglb: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Gbgpihihhal {
    #[serde(rename = "ACTIVITY_BANNER_MONSTER")]
    ActivityBannerMonster,
}

pub fn load() -> Result<ActivityBannerExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityBannerExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
