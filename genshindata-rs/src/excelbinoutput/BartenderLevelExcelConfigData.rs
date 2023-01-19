// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type BartenderLevelExcelConfigData = Vec<BartenderLevelExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BartenderLevelExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,

    #[serde(rename = "BDHGNMNHEFG")]
    pub bdhgnmnhefg: i64,

    #[serde(rename = "KPDLEHHJCCL")]
    pub kpdlehhjccl: i64,

    #[serde(rename = "IGIPAAHLKNF")]
    pub igipaahlknf: Vec<i64>,

    #[serde(rename = "HGGDOEJKBFG")]
    pub hggdoejkbfg: i64,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "KKOIHFJIIBN")]
    pub kkoihfjiibn: i64,
}

pub fn load() -> Result<BartenderLevelExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "BartenderLevelExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
