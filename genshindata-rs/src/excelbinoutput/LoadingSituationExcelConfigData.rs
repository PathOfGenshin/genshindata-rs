// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type LoadingSituationExcelConfigData = Vec<LoadingSituationExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadingSituationExcelConfigDatum {
    #[serde(rename = "stageID")]
    pub stage_id: i64,

    #[serde(rename = "AAHEMABIEAL")]
    pub aahemabieal: String,

    #[serde(rename = "PMOBDHCBBPG")]
    pub pmobdhcbbpg: Vec<i64>,

    #[serde(rename = "GMJPCNMOHMB")]
    pub gmjpcnmohmb: Vec<i64>,

    #[serde(rename = "ANPKJEDECBO")]
    pub anpkjedecbo: Option<Anpkjedecbo>,

    #[serde(rename = "picPath")]
    pub pic_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Anpkjedecbo {
    #[serde(rename = "LOADING_AREA_CITY")]
    LoadingAreaCity,

    #[serde(rename = "LOADING_AREA_OUTDOOR")]
    LoadingAreaOutdoor,
}

pub fn load() -> Result<LoadingSituationExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "LoadingSituationExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
