// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivityMuqadasPotionLevelExcelConfigData = Vec<ActivityMuqadasPotionLevelExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityMuqadasPotionLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "CAOIJLGKBAH")]
    pub caoijlgkbah: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "HCNBJHGCIHC")]
    pub hcnbjhgcihc: Vec<i64>,

    #[serde(rename = "AOAMGBJANDP")]
    pub aoamgbjandp: i64,

    #[serde(rename = "FOMCEGDAJHE")]
    pub fomcegdajhe: i64,

    #[serde(rename = "DKOLHEPIKIP")]
    pub dkolhepikip: i64,

    #[serde(rename = "HEGOLLGGKGM")]
    pub hegollggkgm: String,

    #[serde(rename = "ICJIBHMKMOI")]
    pub icjibhmkmoi: String,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "NLNOHIGMOCI")]
    pub nlnohigmoci: i64,
}

pub fn load() -> Result<ActivityMuqadasPotionLevelExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityMuqadasPotionLevelExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
