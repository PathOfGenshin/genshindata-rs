// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type HuntingClueTextExcelConfigData = Vec<HuntingClueTextExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HuntingClueTextExcelConfigDatum {
    #[serde(rename = "clueTextId")]
    pub clue_text_id: i64,

    #[serde(rename = "reminderIdList")]
    pub reminder_id_list: Vec<i64>,
}

pub fn load() -> Result<HuntingClueTextExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "HuntingClueTextExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
