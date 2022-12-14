// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type SalvageStageDataExcelConfigData = Vec<SalvageStageDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SalvageStageDataExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "JPEKMEMNPMK")]
    pub jpekmemnpmk: i64,

    #[serde(rename = "KNEGGLJKGKA")]
    pub kneggljkgka: i64,

    #[serde(rename = "AGJPJBEMMNG")]
    pub agjpjbemmng: Vec<i64>,

    #[serde(rename = "PADNINBHALP")]
    pub padninbhalp: Vec<i64>,

    #[serde(rename = "PMPLPIFHBMG")]
    pub pmplpifhbmg: Vec<Option<serde_json::Value>>,

    #[serde(rename = "JEEMCFBCLOL")]
    pub jeemcfbclol: Vec<i64>,
}

pub fn load() -> Result<SalvageStageDataExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "SalvageStageDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
