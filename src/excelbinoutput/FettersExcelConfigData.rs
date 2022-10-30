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

pub type FettersExcelConfigData = Vec<FettersExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FettersExcelConfigDatum {
    #[serde(rename = "MEHNNNCLHLI")]
    pub mehnnnclhli: i64,

    #[serde(rename = "OALKEOBGAHC")]
    pub oalkeobgahc: Vec<i64>,

    #[serde(rename = "finishConds")]
    pub finish_conds: Vec<i64>,

    #[serde(rename = "tips")]
    pub tips: Vec<i64>,

    #[serde(rename = "voiceTitleTextMapHash")]
    pub voice_title_text_map_hash: i64,

    #[serde(rename = "voiceFile")]
    pub voice_file: String,

    #[serde(rename = "voiceFileTextTextMapHash")]
    pub voice_file_text_text_map_hash: i64,

    #[serde(rename = "voiceTitleLockedTextMapHash")]
    pub voice_title_locked_text_map_hash: i64,

    #[serde(rename = "fetterId")]
    pub fetter_id: i64,

    #[serde(rename = "avatarId")]
    pub avatar_id: i64,

    #[serde(rename = "openConds")]
    pub open_conds: Vec<OpenCond>,

    #[serde(rename = "KIENFJBHKEP")]
    pub kienfjbhkep: Vec<Option<serde_json::Value>>,

    #[serde(rename = "isHiden")]
    pub is_hiden: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct OpenCond {
    #[serde(rename = "condType")]
    pub cond_type: Option<CondType>,

    #[serde(rename = "paramList")]
    pub param_list: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum CondType {
    #[serde(rename = "FETTER_COND_AVATAR_PROMOTE_LEVEL")]
    FetterCondAvatarPromoteLevel,

    #[serde(rename = "FETTER_COND_FETTER_LEVEL")]
    FetterCondFetterLevel,

    #[serde(rename = "FETTER_COND_FINISH_PARENT_QUEST")]
    FetterCondFinishParentQuest,

    #[serde(rename = "FETTER_COND_FINISH_QUEST")]
    FetterCondFinishQuest,

    #[serde(rename = "FETTER_COND_NOT_OPEN")]
    FetterCondNotOpen,

    #[serde(rename = "FETTER_COND_PLAYER_BIRTHDAY")]
    FetterCondPlayerBirthday,

    #[serde(rename = "FETTER_COND_UNLOCK_TRANS_POINT")]
    FetterCondUnlockTransPoint,
}
