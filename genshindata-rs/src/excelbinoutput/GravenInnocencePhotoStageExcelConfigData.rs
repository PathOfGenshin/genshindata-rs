// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GravenInnocencePhotoStageExcelConfigData = Vec<GravenInnocencePhotoStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GravenInnocencePhotoStageExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "OIDGCKBJOBE")]
    pub oidgckbjobe: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "IEHDLFOOEAB")]
    pub iehdlfooeab: i64,

    #[serde(rename = "infoDescTextMapHash")]
    pub info_desc_text_map_hash: i64,

    #[serde(rename = "GLNBFLNKPIP")]
    pub glnbflnkpip: Vec<i64>,

    #[serde(rename = "GNNHGCFPCFI")]
    pub gnnhgcfpcfi: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "DBPLLCGLHBC")]
    pub dbpllcglhbc: i64,

    #[serde(rename = "LLBPIOBNHBA")]
    pub llbpiobnhba: Vec<i64>,

    #[serde(rename = "CELIJOOEOII")]
    pub celijooeoii: Option<i64>,
}

pub fn load() -> Result<GravenInnocencePhotoStageExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GravenInnocencePhotoStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
