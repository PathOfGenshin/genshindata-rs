/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type CustomLevelTagConfigData = Vec<CustomLevelTagConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomLevelTagConfigDatum {
    pub config_id: i64,
    pub tag_title_text_map_hash: i64,
    pub sort_id: i64,
    pub is_default: Option<bool>,
}
