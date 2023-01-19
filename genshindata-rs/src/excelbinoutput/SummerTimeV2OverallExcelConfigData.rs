// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type SummerTimeV2OverallExcelConfigData = Vec<SummerTimeV2OverallExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummerTimeV2OverallExcelConfigDatum {
    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "ADHFBKFHBIN")]
    pub adhfbkfhbin: i64,

    #[serde(rename = "itemId")]
    pub item_id: i64,
}

pub fn load() -> Result<SummerTimeV2OverallExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "SummerTimeV2OverallExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
