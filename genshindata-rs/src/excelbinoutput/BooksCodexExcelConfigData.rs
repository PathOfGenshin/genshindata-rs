/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BooksCodexExcelConfigData = Vec<BooksCodexExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BooksCodexExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,
    pub material_id: i64,
    #[serde(rename = "SortOrder")]
    pub sort_order: i64,
    pub is_disuse: Option<bool>,
}
