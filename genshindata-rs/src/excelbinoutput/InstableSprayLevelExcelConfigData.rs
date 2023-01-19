// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type InstableSprayLevelExcelConfigData = Vec<InstableSprayLevelExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct InstableSprayLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "HGHDONCFIMD")]
    pub hghdoncfimd: Vec<i64>,

    #[serde(rename = "ADKNIIAKNGO")]
    pub adkniiakngo: Vec<i64>,

    #[serde(rename = "HCMOOOJOFLO")]
    pub hcmooojoflo: Vec<i64>,

    #[serde(rename = "KDJOHGJOJHB")]
    pub kdjohgjojhb: String,
}

pub fn load() -> Result<InstableSprayLevelExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "InstableSprayLevelExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
