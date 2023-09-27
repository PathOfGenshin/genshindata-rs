/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type OpActivityBonusExcelConfigData = Vec<OpActivityBonusExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpActivityBonusExcelConfigDatum {
    pub bonus_id: i64,
    pub source_type: SourceType,
    pub source_param: String,
    pub open_level: i64,
    pub bonus_ratio: i64,
    pub text_map_id_list: Vec<String>,
    pub track_para: Vec<Option<serde_json::Value>>,
    pub icon_background: i64,
    pub icon_foreground: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SourceType {
    #[serde(rename = "SOURCE_TYPE_BLOSSOM")]
    SourceTypeBlossom,
    #[serde(rename = "SOURCE_TYPE_DUNGEON")]
    SourceTypeDungeon,
}
