// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type NewActivityTimeGroupExcelConfigData = Vec<NewActivityTimeGroupExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewActivityTimeGroupExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "groupIdList")]
    pub group_id_list: Vec<i64>,

    #[serde(rename = "duration")]
    pub duration: Vec<i64>,
}

pub fn load() -> Result<NewActivityTimeGroupExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "NewActivityTimeGroupExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
