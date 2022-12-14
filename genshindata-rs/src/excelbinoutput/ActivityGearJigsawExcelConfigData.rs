// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivityGearJigsawExcelConfigData = Vec<ActivityGearJigsawExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityGearJigsawExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "LDBPEBKBLDO")]
    pub ldbpebkbldo: i64,

    #[serde(rename = "GPEGMJKNDJL")]
    pub gpegmjkndjl: String,

    #[serde(rename = "LCILDKAHBEH")]
    pub lcildkahbeh: String,

    #[serde(rename = "IAEFNMENABD")]
    pub iaefnmenabd: String,

    #[serde(rename = "FCIIIDIPEIF")]
    pub fciiidipeif: i64,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,
}

pub fn load() -> Result<ActivityGearJigsawExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityGearJigsawExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
