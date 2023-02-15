// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ViewCodexExcelConfigData = Vec<ViewCodexExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewCodexExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "gadgetId")]
    pub gadget_id: i64,

    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "image")]
    pub image: String,

    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "worldAreaId")]
    pub world_area_id: i64,

    #[serde(rename = "SortOrder")]
    pub sort_order: i64,

    #[serde(rename = "showOnlyUnlocked")]
    pub show_only_unlocked: Option<bool>,
}

pub fn load() -> Result<ViewCodexExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ViewCodexExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
