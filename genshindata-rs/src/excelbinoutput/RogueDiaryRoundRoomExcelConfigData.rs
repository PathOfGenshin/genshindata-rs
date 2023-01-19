// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type RogueDiaryRoundRoomExcelConfigData = Vec<RogueDiaryRoundRoomExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RogueDiaryRoundRoomExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "ENMKLCGEIOF")]
    pub enmklcgeiof: Option<i64>,

    #[serde(rename = "CCCEJNIDCOG")]
    pub cccejnidcog: Vec<Cccejnidcog>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cccejnidcog {
    #[serde(rename = "IJDMLPJHPNK")]
    pub ijdmlpjhpnk: Vec<i64>,

    #[serde(rename = "LODPCKONJOP")]
    pub lodpckonjop: Vec<i64>,
}

pub fn load() -> Result<RogueDiaryRoundRoomExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "RogueDiaryRoundRoomExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
