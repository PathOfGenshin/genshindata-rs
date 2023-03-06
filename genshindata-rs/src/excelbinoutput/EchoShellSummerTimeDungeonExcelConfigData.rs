// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type EchoShellSummerTimeDungeonExcelConfigData = Vec<EchoShellSummerTimeDungeonExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoShellSummerTimeDungeonExcelConfigDatum {
    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "GDLGGMDMIOO")]
    pub gdlggmdmioo: i64,

    #[serde(rename = "NNDJLCLCJPD")]
    pub nndjlclcjpd: Option<i64>,
}

pub fn load() -> Result<EchoShellSummerTimeDungeonExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "EchoShellSummerTimeDungeonExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
