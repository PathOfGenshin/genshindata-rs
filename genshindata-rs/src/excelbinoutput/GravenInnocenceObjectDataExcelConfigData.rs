/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GravenInnocenceObjectDataExcelConfigData = Vec<GravenInnocenceObjectDataExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GravenInnocenceObjectDataExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "groupBundleID")]
    pub group_bundle_id: Option<i64>,
    pub object_index: i64,
    pub watcher: i64,
    pub picture: String,
    pub desc_text_map_hash: i64,
    pub group_desc_text_map_hash: i64,
}
