// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MusicInstrumentConfigData = Vec<MusicInstrumentConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MusicInstrumentConfigDatum {
    #[serde(rename = "JILPKMCILIM")]
    pub jilpkmcilim: i64,

    #[serde(rename = "OIPFLLCBBBG")]
    pub oipfllcbbbg: i64,

    #[serde(rename = "GEDPLNBBAAJ")]
    pub gedplnbbaaj: i64,

    #[serde(rename = "IGJALPMGIPB")]
    pub igjalpmgipb: String,

    #[serde(rename = "DIEDIBLNIEP")]
    pub diediblniep: String,

    #[serde(rename = "CONMLLOHLDB")]
    pub conmllohldb: String,

    #[serde(rename = "LPKBEAEHGDE")]
    pub lpkbeaehgde: String,
}

pub fn load() -> Result<MusicInstrumentConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MusicInstrumentConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
