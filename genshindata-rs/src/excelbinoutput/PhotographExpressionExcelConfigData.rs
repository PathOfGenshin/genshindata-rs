// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type PhotographExpressionExcelConfigData = Vec<PhotographExpressionExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotographExpressionExcelConfigDatum {
    #[serde(rename = "emotionName")]
    pub emotion_name: String,

    #[serde(rename = "phonemeName")]
    pub phoneme_name: String,

    #[serde(rename = "icon")]
    pub icon: Icon,

    #[serde(rename = "emotionDescriptionTextMapHash")]
    pub emotion_description_text_map_hash: i64,

    #[serde(rename = "unlockDescTextMapHash")]
    pub unlock_desc_text_map_hash: i64,

    #[serde(rename = "fetterId")]
    pub fetter_id: i64,

    #[serde(rename = "avatarId")]
    pub avatar_id: i64,

    #[serde(rename = "openConds")]
    pub open_conds: Vec<OpenCond>,

    #[serde(rename = "JPOEJCIDIFK")]
    pub jpoejcidifk: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenCond {
    #[serde(rename = "paramList")]
    pub param_list: Vec<i64>,

    #[serde(rename = "condType")]
    pub cond_type: Option<CondType>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum CondType {
    #[serde(rename = "FETTER_COND_FETTER_LEVEL")]
    FetterCondFetterLevel,
}

pub fn load() -> Result<PhotographExpressionExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "PhotographExpressionExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
