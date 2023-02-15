// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type RegionSearchCondExcelConfigData = Vec<RegionSearchCondExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionSearchCondExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "searchNameTextMapHash")]
    pub search_name_text_map_hash: i64,

    #[serde(rename = "searchDescTextMapHash")]
    pub search_desc_text_map_hash: i64,

    #[serde(rename = "searchMapDescTextMapHash")]
    pub search_map_desc_text_map_hash: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "logicType")]
    pub logic_type: String,

    #[serde(rename = "cond")]
    pub cond: Vec<Cond>,

    #[serde(rename = "regionList")]
    pub region_list: Vec<i64>,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "totalProgress")]
    pub total_progress: i64,

    #[serde(rename = "reminderId")]
    pub reminder_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cond {
    #[serde(rename = "type")]
    pub cond_type: Option<String>,

    #[serde(rename = "param")]
    pub param: Vec<i64>,
}

pub fn load() -> Result<RegionSearchCondExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "RegionSearchCondExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
