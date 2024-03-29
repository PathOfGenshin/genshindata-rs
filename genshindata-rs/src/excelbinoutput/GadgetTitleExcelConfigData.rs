/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GadgetTitleExcelConfigData = Vec<GadgetTitleExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GadgetTitleExcelConfigDatum {
    pub gadget_id: i64,
    pub title_text_map_hash: i64,
    pub name_text_map_hash: i64,
}
