// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

pub type PhotographExpressionExcelConfigData = Vec<PhotographExpressionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
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

    #[serde(rename = "KIENFJBHKEP")]
    pub kienfjbhkep: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct OpenCond {
    #[serde(rename = "paramList")]
    pub param_list: Vec<i64>,

    #[serde(rename = "condType")]
    pub cond_type: Option<CondType>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub enum CondType {
    #[serde(rename = "FETTER_COND_FETTER_LEVEL")]
    FetterCondFetterLevel,
}
