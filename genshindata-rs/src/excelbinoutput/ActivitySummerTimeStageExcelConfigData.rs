// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivitySummerTimeStageExcelConfigData = Vec<ActivitySummerTimeStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivitySummerTimeStageExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "mainQuest")]
    pub main_quest: i64,

    #[serde(rename = "OJPNDEAAGND")]
    pub ojpndeaagnd: Option<i64>,

    #[serde(rename = "NPOOHGEINFE")]
    pub npoohgeinfe: Option<i64>,

    #[serde(rename = "EHDEMGOKAML")]
    pub ehdemgokaml: i64,

    #[serde(rename = "JDFEPFFJNCO")]
    pub jdfepffjnco: i64,

    #[serde(rename = "IJEGBCHLBLG")]
    pub ijegbchlblg: i64,

    #[serde(rename = "DFHCAPBCAEO")]
    pub dfhcapbcaeo: i64,

    #[serde(rename = "preQuest")]
    pub pre_quest: Option<i64>,
}

pub fn load() -> Result<ActivitySummerTimeStageExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivitySummerTimeStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
