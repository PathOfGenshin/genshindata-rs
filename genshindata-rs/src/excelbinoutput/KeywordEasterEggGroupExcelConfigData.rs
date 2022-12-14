// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type KeywordEasterEggGroupExcelConfigData = Vec<KeywordEasterEggGroupExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeywordEasterEggGroupExcelConfigDatum {
    #[serde(rename = "groupID")]
    pub group_id: i64,

    #[serde(rename = "logicType")]
    pub logic_type: String,

    #[serde(rename = "HJDLNPEGNEB")]
    pub hjdlnpegneb: Vec<i64>,
}

pub fn load() -> Result<KeywordEasterEggGroupExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "KeywordEasterEggGroupExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
