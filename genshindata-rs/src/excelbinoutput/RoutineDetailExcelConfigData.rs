// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type RoutineDetailExcelConfigData = Vec<RoutineDetailExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoutineDetailExcelConfigDatum {
    #[serde(rename = "routineId")]
    pub routine_id: i64,

    #[serde(rename = "routineType")]
    pub routine_type: RoutineType,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "isBackup")]
    pub is_backup: Option<bool>,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "finishContent")]
    pub finish_content: FinishContent,

    #[serde(rename = "goodsIdVec")]
    pub goods_id_vec: Vec<Phnllgnaada>,

    #[serde(rename = "PHNLLGNAADA")]
    pub phnllgnaada: Vec<Phnllgnaada>,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "goalTextMapHash")]
    pub goal_text_map_hash: i64,

    #[serde(rename = "centerPosition")]
    pub center_position: String,

    #[serde(rename = "radarRadius")]
    pub radar_radius: i64,

    #[serde(rename = "enterDistance")]
    pub enter_distance: i64,

    #[serde(rename = "exitDistance")]
    pub exit_distance: i64,

    #[serde(rename = "tag")]
    pub tag: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinishContent {
    #[serde(rename = "finishType")]
    pub finish_type: FinishType,

    #[serde(rename = "progress")]
    pub progress: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Phnllgnaada {
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FinishType {
    #[serde(rename = "ROUTINE_CHALLENGE_FINISH")]
    RoutineChallengeFinish,

    #[serde(rename = "ROUTINE_FINISH_KILL_MONSTER")]
    RoutineFinishKillMonster,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RoutineType {
    #[serde(rename = "ROUTINE_SNOW_MOUNTAIN")]
    RoutineSnowMountain,
}

pub fn load() -> Result<RoutineDetailExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "RoutineDetailExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
