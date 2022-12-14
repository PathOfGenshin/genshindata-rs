// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type WinterCampRaceExcelConfigData = Vec<WinterCampRaceExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WinterCampRaceExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "JICEOLNKCME")]
    pub jiceolnkcme: i64,

    #[serde(rename = "KEALFKBKPPG")]
    pub kealfkbkppg: Vec<i64>,

    #[serde(rename = "EOGHHDGEHPK")]
    pub eoghhdgehpk: Vec<i64>,

    #[serde(rename = "LAPBBGEDIJM")]
    pub lapbbgedijm: i64,

    #[serde(rename = "PJJMFLFKCEE")]
    pub pjjmflfkcee: i64,

    #[serde(rename = "LCDMCLFKCDG")]
    pub lcdmclfkcdg: i64,

    #[serde(rename = "FOLMAJDIJLJ")]
    pub folmajdijlj: Vec<i64>,
}

pub fn load() -> Result<WinterCampRaceExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "WinterCampRaceExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
