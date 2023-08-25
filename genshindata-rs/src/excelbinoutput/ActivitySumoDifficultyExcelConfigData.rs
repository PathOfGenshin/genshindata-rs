/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivitySumoDifficultyExcelConfigData = Vec<ActivitySumoDifficultyExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActivitySumoDifficultyExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "FBDAGDPMAJG")]
    pub fbdagdpmajg: i64,
    #[serde(rename = "DCLJMHKLLDP")]
    pub dcljmhklldp: i64,
    #[serde(rename = "JLEOEHNJHLI")]
    pub jleoehnjhli: i64,
    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
    #[serde(rename = "difficulty")]
    pub difficulty: Option<String>,
}
