// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type TalkSelectTimeOutExcelConfigData = Vec<Option<serde_json::Value>>;

pub fn load() -> Result<TalkSelectTimeOutExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "TalkSelectTimeOutExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
