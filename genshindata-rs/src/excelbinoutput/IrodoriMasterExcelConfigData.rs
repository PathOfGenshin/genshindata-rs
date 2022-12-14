// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type IrodoriMasterExcelConfigData = Vec<IrodoriMasterExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct IrodoriMasterExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "levelID")]
    pub level_id: i64,

    #[serde(rename = "EFEOOJFNKLN")]
    pub efeoojfnkln: Efeoojfnkln,

    #[serde(rename = "EGKHABGENGP")]
    pub egkhabgengp: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "KDNPDJMCFFC")]
    pub kdnpdjmcffc: i64,

    #[serde(rename = "IGBECLANHOE")]
    pub igbeclanhoe: i64,

    #[serde(rename = "condID")]
    pub cond_id: i64,

    #[serde(rename = "GDOOKAENGIN")]
    pub gdookaengin: i64,

    #[serde(rename = "AMILCACOMFN")]
    pub amilcacomfn: i64,

    #[serde(rename = "OKEEFLOAMMO")]
    pub okeefloammo: i64,

    #[serde(rename = "IEGAFHCPJMG")]
    pub iegafhcpjmg: i64,

    #[serde(rename = "GNBEOFKIKEJ")]
    pub gnbeofkikej: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Efeoojfnkln {
    #[serde(rename = "IRODORI_MASTER_LEVEL_HARD")]
    IrodoriMasterLevelHard,

    #[serde(rename = "IRODORI_MASTER_LEVEL_MASTER")]
    IrodoriMasterLevelMaster,

    #[serde(rename = "IRODORI_MASTER_LEVEL_NORAML")]
    IrodoriMasterLevelNoraml,
}

pub fn load() -> Result<IrodoriMasterExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "IrodoriMasterExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
