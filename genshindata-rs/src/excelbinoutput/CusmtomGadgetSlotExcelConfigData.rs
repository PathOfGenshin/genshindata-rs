// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type CusmtomGadgetSlotExcelConfigData = Vec<CusmtomGadgetSlotExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CusmtomGadgetSlotExcelConfigDatum {
    #[serde(rename = "POMEJABCDBC")]
    pub pomejabcdbc: i64,

    #[serde(rename = "ENKPMGGPBEN")]
    pub enkpmggpben: Vec<i64>,

    #[serde(rename = "IPCGHOPCGEI")]
    pub ipcghopcgei: Option<bool>,

    #[serde(rename = "GADMOLLAIJB")]
    pub gadmollaijb: Vec<i64>,

    #[serde(rename = "JIBKILIJFJD")]
    pub jibkilijfjd: Vec<i64>,

    #[serde(rename = "OCCHOKMGLCF")]
    pub occhokmglcf: Option<i64>,
}

pub fn load() -> Result<CusmtomGadgetSlotExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "CusmtomGadgetSlotExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
