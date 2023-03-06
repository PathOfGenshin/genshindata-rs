// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type TreasureMapBonusRegionExcelConfigData = Vec<TreasureMapBonusRegionExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TreasureMapBonusRegionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "NMGCJGHCKHM")]
    pub nmgcjghckhm: i64,

    #[serde(rename = "reviseLevel")]
    pub revise_level: i64,

    #[serde(rename = "mapTitleTextMapHash")]
    pub map_title_text_map_hash: i64,

    #[serde(rename = "mapDescTextMapHash")]
    pub map_desc_text_map_hash: i64,

    #[serde(rename = "showImage")]
    pub show_image: String,

    #[serde(rename = "HAMIMICHNBF")]
    pub hamimichnbf: i64,

    #[serde(rename = "regionCenter")]
    pub region_center: Vec<f64>,

    #[serde(rename = "regionRadius")]
    pub region_radius: i64,

    #[serde(rename = "unlockRegionId")]
    pub unlock_region_id: Option<i64>,
}

pub fn load() -> Result<TreasureMapBonusRegionExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "TreasureMapBonusRegionExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
