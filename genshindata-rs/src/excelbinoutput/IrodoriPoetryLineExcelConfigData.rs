/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type IrodoriPoetryLineExcelConfigData = Vec<IrodoriPoetryLineExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IrodoriPoetryLineExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,
    pub content_text_map_hash: i64,
    pub var: Option<i64>,
}
