/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityExcelConfigData = Vec<ActivityExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActivityExcelConfigDatum {
    pub activity_id: i64,
    #[serde(rename = "Type")]
    pub activity_excel_config_datum_type: String,
    pub destroy_item: Vec<DestroyItem>,
    pub name_text_map_hash: i64,
    pub activity_scene_tag: String,
    pub is_load_terrain: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DestroyItem {
    pub item_id: Vec<i64>,
}
