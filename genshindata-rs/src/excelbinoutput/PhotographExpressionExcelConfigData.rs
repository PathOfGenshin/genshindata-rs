/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type PhotographExpressionExcelConfigData = Vec<PhotographExpressionExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhotographExpressionExcelConfigDatum {
    pub emotion_name: String,
    pub phoneme_name: String,
    pub icon: Icon,
    pub emotion_description_text_map_hash: i64,
    pub unlock_desc_text_map_hash: i64,
    pub fetter_id: i64,
    pub avatar_id: i64,
    pub open_conds: Vec<OpenCond>,
    #[serde(rename = "DLPFLPGMMNA")]
    pub dlpflpgmmna: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Icon {
    #[serde(rename = "UI_EmotIcon_Blink")]
    UiEmotIconBlink,
    #[serde(rename = "UI_EmotIcon_Confidence")]
    UiEmotIconConfidence,
    #[serde(rename = "UI_EmotIcon_Earnest")]
    UiEmotIconEarnest,
    #[serde(rename = "UI_EmotIcon_Expect")]
    UiEmotIconExpect,
    #[serde(rename = "UI_EmotIcon_Happy")]
    UiEmotIconHappy,
    #[serde(rename = "UI_EmotIcon_Lose")]
    UiEmotIconLose,
    #[serde(rename = "UI_EmotIcon_Normal")]
    UiEmotIconNormal,
    #[serde(rename = "UI_EmotIcon_Ponder")]
    UiEmotIconPonder,
    #[serde(rename = "UI_EmotIcon_Smile")]
    UiEmotIconSmile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCond {
    pub param_list: Vec<i64>,
    pub cond_type: Option<CondType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CondType {
    #[serde(rename = "FETTER_COND_FETTER_LEVEL")]
    FetterCondFetterLevel,
}
