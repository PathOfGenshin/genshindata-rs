// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ForgeExcelConfigData = Vec<ForgeExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ForgeExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "playerLevel")]
    pub player_level: i64,

    #[serde(rename = "isDefaultShow")]
    pub is_default_show: Option<bool>,

    #[serde(rename = "effectiveWorldLevels")]
    pub effective_world_levels: Vec<i64>,

    #[serde(rename = "forgeType")]
    pub forge_type: i64,

    #[serde(rename = "showItemId")]
    pub show_item_id: i64,

    #[serde(rename = "JCEPJOHDFHM")]
    pub jcepjohdfhm: Option<i64>,

    #[serde(rename = "resultItemId")]
    pub result_item_id: Option<i64>,

    #[serde(rename = "resultItemCount")]
    pub result_item_count: i64,

    #[serde(rename = "forgeTime")]
    pub forge_time: i64,

    #[serde(rename = "queueNum")]
    pub queue_num: i64,

    #[serde(rename = "scoinCost")]
    pub scoin_cost: i64,

    #[serde(rename = "randomItems")]
    pub random_items: Vec<RandomItem>,

    #[serde(rename = "materialItems")]
    pub material_items: Vec<MaterialItem>,

    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "forgePoint")]
    pub forge_point: Option<i64>,

    #[serde(rename = "forgePointNoticeTextMapHash")]
    pub forge_point_notice_text_map_hash: i64,

    #[serde(rename = "mainRandomDropId")]
    pub main_random_drop_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialItem {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomItem {
}

pub fn load() -> Result<ForgeExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ForgeExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
