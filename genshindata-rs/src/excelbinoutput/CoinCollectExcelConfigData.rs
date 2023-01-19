// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type CoinCollectExcelConfigData = Vec<CoinCollectExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinCollectExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "dayIndex")]
    pub day_index: i64,

    #[serde(rename = "CNCFLOOMOMF")]
    pub cncfloomomf: i64,

    #[serde(rename = "IHKMHDDFFJA")]
    pub ihkmhddffja: i64,

    #[serde(rename = "draftId")]
    pub draft_id: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "LKLMBIJGJIP")]
    pub lklmbijgjip: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "HKEOFEEKOOL")]
    pub hkeofeekool: i64,

    #[serde(rename = "OHNAPLDNIJL")]
    pub ohnapldnijl: Vec<i64>,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "GFDFJMMHMIC")]
    pub gfdfjmmhmic: i64,

    #[serde(rename = "OKPLOMJEIIB")]
    pub okplomjeiib: i64,

    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,

    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,

    #[serde(rename = "DMEOKDENLMA")]
    pub dmeokdenlma: i64,

    #[serde(rename = "ODGJEDHADAK")]
    pub odgjedhadak: Vec<i64>,

    #[serde(rename = "OJHIKLAHHFK")]
    pub ojhiklahhfk: String,
}

pub fn load() -> Result<CoinCollectExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "CoinCollectExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
