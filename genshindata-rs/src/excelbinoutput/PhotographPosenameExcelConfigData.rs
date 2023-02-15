// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type PhotographPosenameExcelConfigData = Vec<PhotographPosenameExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotographPosenameExcelConfigDatum {
    #[serde(rename = "poseFile")]
    pub pose_file: PoseFile,

    #[serde(rename = "poseIcon")]
    pub pose_icon: PoseIcon,

    #[serde(rename = "poseNameTextMapHash")]
    pub pose_name_text_map_hash: i64,

    #[serde(rename = "unlockDescTextMapHash")]
    pub unlock_desc_text_map_hash: i64,

    #[serde(rename = "fetterId")]
    pub fetter_id: i64,

    #[serde(rename = "avatarId")]
    pub avatar_id: i64,

    #[serde(rename = "openConds")]
    pub open_conds: Vec<OpenCond>,

    #[serde(rename = "NKHEPEJMMDG")]
    pub nkhepejmmdg: Vec<Option<serde_json::Value>>,

    #[serde(rename = "animatorstateId")]
    pub animatorstate_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenCond {
    #[serde(rename = "paramList")]
    pub param_list: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PoseFile {
    #[serde(rename = "Akimbo02")]
    Akimbo02,

    #[serde(rename = "Standby")]
    Standby,

    #[serde(rename = "Think01")]
    Think01,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PoseIcon {
    #[serde(rename = "UI_PoseIcon_Akimbo")]
    UiPoseIconAkimbo,

    #[serde(rename = "UI_PoseIcon_Default")]
    UiPoseIconDefault,

    #[serde(rename = "UI_PoseIcon_Thinking")]
    UiPoseIconThinking,
}

pub fn load() -> Result<PhotographPosenameExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "PhotographPosenameExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
