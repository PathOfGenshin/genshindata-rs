// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type SubSpriteTagExcelConfigData = Vec<SubSpriteTagExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubSpriteTagExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "PLDDKCOBEAP")]
    pub plddkcobeap: String,

    #[serde(rename = "GPAJLFGKMDG")]
    pub gpajlfgkmdg: f64,

    #[serde(rename = "FLHKCABAOLO")]
    pub flhkcabaolo: Option<f64>,

    #[serde(rename = "OEBEGBDACPF")]
    pub oebegbdacpf: Option<f64>,
}

pub fn load() -> Result<SubSpriteTagExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "SubSpriteTagExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
