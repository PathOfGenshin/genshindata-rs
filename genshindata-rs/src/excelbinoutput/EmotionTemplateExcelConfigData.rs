/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type EmotionTemplateExcelConfigData = Vec<EmotionTemplateExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmotionTemplateExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,
    pub free_style_id: i64,
    pub bubble_name: String,
    pub emotion_path: String,
    pub protect_time: f64,
}
