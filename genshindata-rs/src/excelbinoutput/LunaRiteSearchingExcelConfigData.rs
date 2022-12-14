// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type LunaRiteSearchingExcelConfigData = Vec<LunaRiteSearchingExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LunaRiteSearchingExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "GBECKABMACL")]
    pub gbeckabmacl: String,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "ACKGDLGBKID")]
    pub ackgdlgbkid: i64,

    #[serde(rename = "MNCMLAMFHHJ")]
    pub mncmlamfhhj: Vec<i64>,

    #[serde(rename = "regionCenter")]
    pub region_center: Vec<f64>,

    #[serde(rename = "GGHBJAPBCLB")]
    pub gghbjapbclb: i64,

    #[serde(rename = "regionRadius")]
    pub region_radius: f64,

    #[serde(rename = "FNPDEOPMNPA")]
    pub fnpdeopmnpa: i64,

    #[serde(rename = "HNHDNKBHNHJ")]
    pub hnhdnkbhnhj: i64,

    #[serde(rename = "BDBAAMCHGLJ")]
    pub bdbaamchglj: i64,

    #[serde(rename = "KAGIDBCGGDK")]
    pub kagidbcggdk: i64,
}

pub fn load() -> Result<LunaRiteSearchingExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "LunaRiteSearchingExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
