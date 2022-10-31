// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ReputationExploreExcelConfigData = Vec<ReputationExploreExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReputationExploreExcelConfigDatum {
    #[serde(rename = "exploreId")]
    pub explore_id: i64,

    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "exploreProgress")]
    pub explore_progress: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "conditionTextTextMapHash")]
    pub condition_text_text_map_hash: i64,
}

pub fn load() -> Result<ReputationExploreExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ReputationExploreExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}