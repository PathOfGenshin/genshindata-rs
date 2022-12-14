// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type MoonfinTrialExcelConfigData = Vec<MoonfinTrialExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MoonfinTrialExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "DJEKBPJHPKA")]
    pub djekbpjhpka: Vec<i64>,

    #[serde(rename = "FINEMIHEDEP")]
    pub finemihedep: i64,

    #[serde(rename = "GKNPALILPCD")]
    pub gknpalilpcd: i64,

    #[serde(rename = "LAGKIHDLBKF")]
    pub lagkihdlbkf: i64,

    #[serde(rename = "ABDGPNDIMPA")]
    pub abdgpndimpa: i64,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,

    #[serde(rename = "DHEHFPOHMKM")]
    pub dhehfpohmkm: Vec<i64>,

    #[serde(rename = "PMJIAJFJEAC")]
    pub pmjiajfjeac: Vec<i64>,
}

pub fn load() -> Result<MoonfinTrialExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MoonfinTrialExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
