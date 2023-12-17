/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type InferenceWordExcelConfigData = Vec<InferenceWordExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InferenceWordExcelConfigDatum {
    pub word_id: i64,
    pub page_id: i64,
    pub default_unlock: Option<bool>,
    pub is_can_interpret: Option<bool>,
    pub interpret_action: Vec<Action>,
    pub is_can_associate: Option<bool>,
    pub associate_word_id: Option<i64>,
    pub associate_action: Vec<Action>,
    pub word_name_text_map_hash: i64,
    pub interpret_dialog_id: Option<i64>,
    pub associate_error_dialog_id: Option<i64>,
    pub freestyle_id: Option<i64>,
    #[serde(rename = "JPOGMJGGIAL")]
    pub jpogmjggial: Vec<i64>,
    #[serde(rename = "LJOIFKJIOIM")]
    pub ljoifkjioim: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "type")]
    pub action_type: Option<Type>,
    pub param: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    #[serde(rename = "INFERENCE_ACTION_UNLOCK_WORD")]
    InferenceActionUnlockWord,
}
