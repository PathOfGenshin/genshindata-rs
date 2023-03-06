// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityHachiFinalStageExcelConfigData = Vec<ActivityHachiFinalStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityHachiFinalStageExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "questId")]
    pub quest_id: Vec<i64>,

    #[serde(rename = "EHDEMGOKAML")]
    pub ehdemgokaml: i64,

    #[serde(rename = "JDFEPFFJNCO")]
    pub jdfepffjnco: i64,

    #[serde(rename = "PEOAACMJNMC")]
    pub peoaacmjnmc: i64,

    #[serde(rename = "FNDOBPOMCLP")]
    pub fndobpomclp: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "FNAONNBLHAB")]
    pub fnaonnblhab: Vec<i64>,

    #[serde(rename = "JGDKIIOLODP")]
    pub jgdkiiolodp: i64,
}

pub fn load() -> Result<ActivityHachiFinalStageExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityHachiFinalStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
