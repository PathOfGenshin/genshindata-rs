// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type BirthdayMailExcelConfigData = Vec<BirthdayMailExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BirthdayMailExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "mailId")]
    pub mail_id: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "effectiveDate")]
    pub effective_date: String,

    #[serde(rename = "DLHBEGHGAJO")]
    pub dlhbeghgajo: i64,
}

pub fn load() -> Result<BirthdayMailExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "BirthdayMailExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
