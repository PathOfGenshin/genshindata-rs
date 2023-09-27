/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type TreasureMapBonusRegionExcelConfigData = Vec<TreasureMapBonusRegionExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreasureMapBonusRegionExcelConfigDatum {
    pub id: i64,
    pub group_id: i64,
    pub reward_id: i64,
    pub reward_preview_id: i64,
    pub revise_level: i64,
    pub map_title_text_map_hash: i64,
    pub map_desc_text_map_hash: i64,
    pub show_image: String,
    pub fragment_num: i64,
    pub region_center: Vec<f64>,
    pub region_radius: i64,
    pub unlock_region_id: Option<i64>,
}
