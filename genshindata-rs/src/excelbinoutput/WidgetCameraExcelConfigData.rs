/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type WidgetCameraExcelConfigData = Vec<WidgetCameraExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetCameraExcelConfigDatum {
    pub id: i64,
    pub hint_distance: i64,
    #[serde(rename = "UIPath")]
    pub ui_path: String,
    pub tips_text_map_hash: i64,
    pub scan_tips_text_map_hash: i64,
    pub scan_success_desc_text_map_hash: i64,
}
