// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivitySpiceStageDataExcelConfigData = Vec<ActivitySpiceStageDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivitySpiceStageDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "HGBGJJPDLLJ")]
    pub hgbgjjpdllj: i64,

    #[serde(rename = "MFJIKKCIPNL")]
    pub mfjikkcipnl: i64,

    #[serde(rename = "EINBMFAHFAJ")]
    pub einbmfahfaj: Vec<i64>,

    #[serde(rename = "BPOLJFKKCIM")]
    pub bpoljfkkcim: Vec<i64>,

    #[serde(rename = "KFKAELBMHOK")]
    pub kfkaelbmhok: Vec<i64>,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "MKNCPALFJLK")]
    pub mkncpalfjlk: f64,

    #[serde(rename = "times")]
    pub times: i64,

    #[serde(rename = "FFDEMFFMICD")]
    pub ffdemffmicd: i64,

    #[serde(rename = "CLFEDHPMDGG")]
    pub clfedhpmdgg: Vec<i64>,

    #[serde(rename = "watcherId")]
    pub watcher_id: i64,
}

pub fn load() -> Result<ActivitySpiceStageDataExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivitySpiceStageDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
