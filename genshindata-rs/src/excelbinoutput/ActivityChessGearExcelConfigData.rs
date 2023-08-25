/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityChessGearExcelConfigData = Vec<ActivityChessGearExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityChessGearExcelConfigDatum {
    #[serde(rename = "gearID")]
    pub gear_id: i64,
    pub gear_name_text_map_hash: i64,
    pub gear_short_name_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub gear_icon_path: String,
    #[serde(rename = "KAIPPGJNHGP")]
    pub kaippgjnhgp: String,
    pub attack: i64,
    pub attack_speed: i64,
    pub attack_range: i64,
    pub build_cost: i64,
    pub demolition_refund: i64,
    pub gadget_id: i64,
    #[serde(rename = "FEMCBPOILEM")]
    pub femcbpoilem: Vec<i64>,
    #[serde(rename = "KNAEPFOBLGP")]
    pub knaepfoblgp: i64,
    #[serde(rename = "EAOHFLDMBCA")]
    pub eaohfldmbca: i64,
    pub is_enable_rotate: Option<bool>,
    #[serde(rename = "DJCHHHBPBGE")]
    pub djchhhbpbge: Option<i64>,
}
