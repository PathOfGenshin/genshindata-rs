// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FettersExcelConfigData = Vec<FettersExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FettersExcelConfigDatum {
    #[serde(rename = "NJNCANOCNJC")]
    pub njncanocnjc: i64,

    #[serde(rename = "BCCFPDJLIFG")]
    pub bccfpdjlifg: Vec<i64>,

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

    #[serde(rename = "NKHEPEJMMDG")]
    pub nkhepejmmdg: Vec<Option<serde_json::Value>>,

    #[serde(rename = "isHiden")]
    pub is_hiden: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenCond {
    #[serde(rename = "condType")]
    pub cond_type: Option<CondType>,

    #[serde(rename = "paramList")]
    pub param_list: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CondType {
    #[serde(rename = "FETTER_COND_AVATAR_PROMOTE_LEVEL")]
    FetterCondAvatarPromoteLevel,

    #[serde(rename = "FETTER_COND_FETTER_LEVEL")]
    FetterCondFetterLevel,

    #[serde(rename = "FETTER_COND_FINISH_PARENT_QUEST")]
    FetterCondFinishParentQuest,

    #[serde(rename = "FETTER_COND_FINISH_QUEST")]
    FetterCondFinishQuest,

    #[serde(rename = "FETTER_COND_PLAYER_BIRTHDAY")]
    FetterCondPlayerBirthday,

    #[serde(rename = "FETTER_COND_UNLOCK_TRANS_POINT")]
    FetterCondUnlockTransPoint,
}

pub fn load() -> Result<FettersExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "FettersExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
