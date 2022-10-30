// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type FetterStoryExcelConfigData = Vec<FetterStoryExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FetterStoryExcelConfigDatum {
    #[serde(rename = "storyTitleTextMapHash")]
    pub story_title_text_map_hash: i64,

    #[serde(rename = "storyContextTextMapHash")]
    pub story_context_text_map_hash: i64,

    #[serde(rename = "storyTitle2TextMapHash")]
    pub story_title2_text_map_hash: i64,

    #[serde(rename = "storyContext2TextMapHash")]
    pub story_context2_text_map_hash: i64,

    #[serde(rename = "tips")]
    pub tips: Vec<i64>,

    #[serde(rename = "storyTitleLockedTextMapHash")]
    pub story_title_locked_text_map_hash: i64,

    #[serde(rename = "fetterId")]
    pub fetter_id: i64,

    #[serde(rename = "avatarId")]
    pub avatar_id: i64,

    #[serde(rename = "openConds")]
    pub open_conds: Vec<Kienfjbhkep>,

    #[serde(rename = "KIENFJBHKEP")]
    pub kienfjbhkep: Vec<Kienfjbhkep>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kienfjbhkep {
    #[serde(rename = "condType")]
    pub cond_type: Option<CondType>,

    #[serde(rename = "paramList")]
    pub param_list: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CondType {
    #[serde(rename = "FETTER_COND_FETTER_LEVEL")]
    FetterCondFetterLevel,

    #[serde(rename = "FETTER_COND_FINISH_PARENT_QUEST")]
    FetterCondFinishParentQuest,

    #[serde(rename = "FETTER_COND_FINISH_QUEST")]
    FetterCondFinishQuest,

    #[serde(rename = "FETTER_COND_NOT_OPEN")]
    FetterCondNotOpen,
}

pub fn load() -> Result<FetterStoryExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "FetterStoryExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
