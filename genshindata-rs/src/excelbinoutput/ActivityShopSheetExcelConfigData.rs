/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityShopSheetExcelConfigData = Vec<ActivityShopSheetExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityShopSheetExcelConfigDatum {
    pub id: i64,
    pub is_ahead_preview: Option<bool>,
    #[serde(rename = "SheetNameTextMapHash")]
    pub sheet_name_text_map_hash: i64,
    pub cond: Vec<Cond>,
    pub sort_level: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cond {
    pub param: Vec<i64>,
    #[serde(rename = "type")]
    pub cond_type: Option<Type>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    #[serde(rename = "ACTIVITY_SHOP_SHEET_COND_TIME_EQUAL_GREATER")]
    ActivityShopSheetCondTimeEqualGreater,
}
