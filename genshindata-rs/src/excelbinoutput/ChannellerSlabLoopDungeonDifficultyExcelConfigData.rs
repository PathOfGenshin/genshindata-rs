// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ChannellerSlabLoopDungeonDifficultyExcelConfigData = Vec<ChannellerSlabLoopDungeonDifficultyExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannellerSlabLoopDungeonDifficultyExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "difficulty")]
    pub difficulty: String,

    #[serde(rename = "scoreRatio")]
    pub score_ratio: f64,

    #[serde(rename = "baseScore")]
    pub base_score: i64,
}

pub fn load() -> Result<ChannellerSlabLoopDungeonDifficultyExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ChannellerSlabLoopDungeonDifficultyExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
