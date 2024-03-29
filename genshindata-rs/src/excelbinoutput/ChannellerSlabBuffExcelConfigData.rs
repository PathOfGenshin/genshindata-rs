/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ChannellerSlabBuffExcelConfigData = Vec<ChannellerSlabBuffExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannellerSlabBuffExcelConfigDatum {
    pub id: i64,
    pub buff_name_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub desc_param: Vec<String>,
    #[serde(rename = "materialID")]
    pub material_id: i64,
    pub icon: String,
    pub buff_quality_type: BuffQualityType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BuffQualityType {
    #[serde(rename = "QUALITY_BLUE")]
    QualityBlue,
    #[serde(rename = "QUALITY_GREEN")]
    QualityGreen,
    #[serde(rename = "QUALITY_PURPLE")]
    QualityPurple,
}
