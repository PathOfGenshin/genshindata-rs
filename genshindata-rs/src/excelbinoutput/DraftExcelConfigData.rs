// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type DraftExcelConfigData = Vec<DraftExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DraftExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "transferConfig")]
    pub transfer_config: Vec<TransferConfig>,

    #[serde(rename = "sceneId")]
    pub scene_id: Option<i64>,

    #[serde(rename = "exec")]
    pub exec: Exec,

    #[serde(rename = "param")]
    pub param: Option<i64>,

    #[serde(rename = "enableMp")]
    pub enable_mp: bool,

    #[serde(rename = "isNeedAllAgree")]
    pub is_need_all_agree: bool,

    #[serde(rename = "confirmCountDown")]
    pub confirm_count_down: i64,

    #[serde(rename = "minPlayerCount")]
    pub min_player_count: i64,

    #[serde(rename = "isNeedTwiceConfirm")]
    pub is_need_twice_confirm: Option<bool>,

    #[serde(rename = "twiceConfirmCountDown")]
    pub twice_confirm_count_down: i64,

    #[serde(rename = "FFIHHDMPHJL")]
    pub ffihhdmphjl: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferConfig {
    #[serde(rename = "groupId")]
    pub group_id: Option<i64>,

    #[serde(rename = "configId")]
    pub config_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Exec {
    #[serde(rename = "DRAFT_EXEC_AUTOSTART_GALLERY")]
    DraftExecAutostartGallery,

    #[serde(rename = "DRAFT_EXEC_BRICK_BREAKER")]
    DraftExecBrickBreaker,

    #[serde(rename = "DRAFT_EXEC_CHAR_AMUSEMENT")]
    DraftExecCharAmusement,

    #[serde(rename = "DRAFT_EXEC_COIN_COLLECT")]
    DraftExecCoinCollect,

    #[serde(rename = "DRAFT_EXEC_GALLERY")]
    DraftExecGallery,

    #[serde(rename = "DRAFT_EXEC_HIDE_AND_SEEK")]
    DraftExecHideAndSeek,
}

pub fn load() -> Result<DraftExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "DraftExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
