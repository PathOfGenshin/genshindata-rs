// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GravenInnocenceRaceLevelExcelConfigData = Vec<GravenInnocenceRaceLevelExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GravenInnocenceRaceLevelExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "JICEOLNKCME")]
    pub jiceolnkcme: i64,

    #[serde(rename = "HPPECOPECAK")]
    pub hppecopecak: Vec<i64>,

    #[serde(rename = "EOGHHDGEHPK")]
    pub eoghhdgehpk: Vec<i64>,

    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,

    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,

    #[serde(rename = "BNKJEJFKLEK")]
    pub bnkjejfklek: i64,

    #[serde(rename = "LAPBBGEDIJM")]
    pub lapbbgedijm: f64,

    #[serde(rename = "PJJMFLFKCEE")]
    pub pjjmflfkcee: f64,
}

pub fn load() -> Result<GravenInnocenceRaceLevelExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GravenInnocenceRaceLevelExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
