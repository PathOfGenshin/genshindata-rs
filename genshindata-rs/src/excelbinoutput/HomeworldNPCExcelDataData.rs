// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type HomeworldNpcExcelDataData = Vec<HomeworldNpcExcelDataDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeworldNpcExcelDataDatum {
    #[serde(rename = "AvatarID")]
    pub avatar_id: i64,

    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "FurnitureID")]
    pub furniture_id: i64,

    #[serde(rename = "PJAPBPDMLAE")]
    pub pjapbpdmlae: i64,

    #[serde(rename = "MJJBGINPDEE")]
    pub mjjbginpdee: Vec<i64>,
}

pub fn load() -> Result<HomeworldNpcExcelDataData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "HomeworldNPCExcelDataData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
