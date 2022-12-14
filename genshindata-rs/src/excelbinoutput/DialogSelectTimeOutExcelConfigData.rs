// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type DialogSelectTimeOutExcelConfigData = Vec<DialogSelectTimeOutExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DialogSelectTimeOutExcelConfigDatum {
    #[serde(rename = "_id")]
    pub id: i64,

    #[serde(rename = "_timeLimit")]
    pub time_limit: f64,

    #[serde(rename = "_nextDialogID")]
    pub next_dialog_id: i64,
}

pub fn load() -> Result<DialogSelectTimeOutExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "DialogSelectTimeOutExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
