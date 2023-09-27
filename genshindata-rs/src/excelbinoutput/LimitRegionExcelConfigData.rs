/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LimitRegionExcelConfigData = Vec<LimitRegionExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitRegionExcelConfigDatum {
    pub id: i64,
    pub name: String,
    pub scene_id: i64,
    #[serde(rename = "type")]
    pub limit_region_excel_config_datum_type: Type,
    pub openstate: Option<String>,
    pub order: Option<i64>,
    #[serde(rename = "reminderID")]
    pub reminder_id: Option<i64>,
    #[serde(rename = "HBMBIBAFJJF")]
    pub hbmbibafjjf: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    #[serde(rename = "LIMIT_REGION_TYPE_ACTIVITY")]
    LimitRegionTypeActivity,
    #[serde(rename = "LIMIT_REGION_TYPE_BIGWORLD")]
    LimitRegionTypeBigworld,
    #[serde(rename = "LIMIT_REGION_TYPE_HOMEWORLD")]
    LimitRegionTypeHomeworld,
}
