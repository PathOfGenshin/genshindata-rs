// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type IrodoriMasterExcelConfigData = Vec<IrodoriMasterExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct IrodoriMasterExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "levelID")]
    pub level_id: i64,

    #[serde(rename = "KMJBNNBFBML")]
    pub kmjbnnbfbml: Kmjbnnbfbml,

    #[serde(rename = "BDHGNMNHEFG")]
    pub bdhgnmnhefg: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "PHDLOLBLDGN")]
    pub phdlolbldgn: i64,

    #[serde(rename = "EAELJOAOALF")]
    pub eaeljoaoalf: i64,

    #[serde(rename = "condID")]
    pub cond_id: i64,

    #[serde(rename = "LMHNMCOJDJD")]
    pub lmhnmcojdjd: i64,

    #[serde(rename = "PMLGMPFDGHN")]
    pub pmlgmpfdghn: i64,

    #[serde(rename = "BJIHGHEJKEG")]
    pub bjihghejkeg: i64,

    #[serde(rename = "LOPAMMNHOLJ")]
    pub lopammnholj: i64,

    #[serde(rename = "DEOCHEPLIHO")]
    pub deochepliho: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Kmjbnnbfbml {
    #[serde(rename = "IRODORI_MASTER_LEVEL_HARD")]
    IrodoriMasterLevelHard,

    #[serde(rename = "IRODORI_MASTER_LEVEL_MASTER")]
    IrodoriMasterLevelMaster,

    #[serde(rename = "IRODORI_MASTER_LEVEL_NORAML")]
    IrodoriMasterLevelNoraml,
}

pub fn load() -> Result<IrodoriMasterExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "IrodoriMasterExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
