// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type PhotographCheckAnimatorDataData = Vec<PhotographCheckAnimatorDataDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotographCheckAnimatorDataDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "DPGKONKALJF")]
    pub dpgkonkaljf: i64,

    #[serde(rename = "DBJGDDINILC")]
    pub dbjgddinilc: Vec<String>,

    #[serde(rename = "NBKOAGEDHAH")]
    pub nbkoagedhah: Vec<Nbkoagedhah>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Nbkoagedhah {
    #[serde(rename = "AVATAR_JUMP")]
    AvatarJump,

    #[serde(rename = "AVATAR_RUN")]
    AvatarRun,

    #[serde(rename = "AVATAR_RUN_TO_IDLE")]
    AvatarRunToIdle,

    #[serde(rename = "AVATAR_RUN_TO_SPRINT")]
    AvatarRunToSprint,

    #[serde(rename = "AVATAR_SPRINT")]
    AvatarSprint,

    #[serde(rename = "AVATAR_SPRINT_TO_IDLE")]
    AvatarSprintToIdle,

    #[serde(rename = "AVATAR_SPRINT_TO_RUN")]
    AvatarSprintToRun,

    #[serde(rename = "AVATAR_SWIM_JUMP")]
    AvatarSwimJump,

    #[serde(rename = "AVATAR_SWIM_JUMP_DROP")]
    AvatarSwimJumpDrop,

    #[serde(rename = "AVATAR_SWIM_JUMP_TO_WATER")]
    AvatarSwimJumpToWater,

    #[serde(rename = "AVATAR_WALK")]
    AvatarWalk,

    #[serde(rename = "AVATAR_WALK_TO_IDLE")]
    AvatarWalkToIdle,
}

pub fn load() -> Result<PhotographCheckAnimatorDataData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "PhotographCheckAnimatorDataData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
