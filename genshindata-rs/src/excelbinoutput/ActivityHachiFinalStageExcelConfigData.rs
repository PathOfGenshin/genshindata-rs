// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivityHachiFinalStageExcelConfigData = Vec<ActivityHachiFinalStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityHachiFinalStageExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "questId")]
    pub quest_id: Vec<i64>,

    #[serde(rename = "DMHGKGCGIJG")]
    pub dmhgkgcgijg: i64,

    #[serde(rename = "PODKEBDBGFJ")]
    pub podkebdbgfj: i64,

    #[serde(rename = "HPFMDAAAPMA")]
    pub hpfmdaaapma: i64,

    #[serde(rename = "LMOLLGLNEHC")]
    pub lmollglnehc: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "FGHMPCNKLIE")]
    pub fghmpcnklie: Vec<i64>,

    #[serde(rename = "MLFHJINKKFM")]
    pub mlfhjinkkfm: i64,
}

pub fn load() -> Result<ActivityHachiFinalStageExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityHachiFinalStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
