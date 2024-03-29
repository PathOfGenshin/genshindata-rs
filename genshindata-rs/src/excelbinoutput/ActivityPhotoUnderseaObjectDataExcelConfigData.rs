/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityPhotoUnderseaObjectDataExcelConfigData = Vec<ActivityPhotoUnderseaObjectDataExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityPhotoUnderseaObjectDataExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "groupBundleID")]
    pub group_bundle_id: Option<i64>,
    pub watcher: i64,
    pub object_index: i64,
    pub desc_text_map_hash: i64,
    #[serde(rename = "EHMJNPBKKBK")]
    pub ehmjnpbkkbk: f64,
    #[serde(rename = "IIGIDIGMICI")]
    pub iigidigmici: Option<bool>,
}
