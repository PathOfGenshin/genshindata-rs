// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityCrystalLinkLevelExcelConfigData = Vec<ActivityCrystalLinkLevelExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityCrystalLinkLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "OBAOGFIILAL")]
    pub obaogfiilal: Vec<i64>,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "BACGDFGGKIG")]
    pub bacgdfggkig: Vec<i64>,

    #[serde(rename = "EJMIIENEJJD")]
    pub ejmiienejjd: Vec<i64>,

    #[serde(rename = "levelTitleTextMapHash")]
    pub level_title_text_map_hash: i64,

    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,

    #[serde(rename = "FINPCANEAKL")]
    pub finpcaneakl: Vec<Finpcaneakl>,

    #[serde(rename = "AGNGGOABDPP")]
    pub agnggoabdpp: Vec<Agnggoabdpp>,

    #[serde(rename = "scoreLevelList")]
    pub score_level_list: Vec<i64>,

    #[serde(rename = "BAAOHHIDGAB")]
    pub baaohhidgab: i64,

    #[serde(rename = "JOOCMELANNI")]
    pub joocmelanni: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Agnggoabdpp {
    #[serde(rename = "FOCFLDNPCNJ")]
    pub focfldnpcnj: Vec<i64>,

    #[serde(rename = "ADKNKJIPKAN")]
    pub adknkjipkan: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Finpcaneakl {
    #[serde(rename = "FOCFLDNPCNJ")]
    pub focfldnpcnj: Vec<String>,

    #[serde(rename = "ADKNKJIPKAN")]
    pub adknkjipkan: Vec<String>,
}

pub fn load() -> Result<ActivityCrystalLinkLevelExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityCrystalLinkLevelExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
