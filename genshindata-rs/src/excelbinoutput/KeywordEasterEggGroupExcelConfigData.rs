/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type KeywordEasterEggGroupExcelConfigData = Vec<KeywordEasterEggGroupExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeywordEasterEggGroupExcelConfigDatum {
    #[serde(rename = "groupID")]
    pub group_id: i64,
    pub logic_type: String,
    #[serde(rename = "APFKJHIEIMO")]
    pub apfkjhieimo: Vec<i64>,
}
