// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivityVintagePresentExcelConfigData = Vec<ActivityVintagePresentExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityVintagePresentExcelConfigDatum {
    #[serde(rename = "IGEGGJAKDGJ")]
    pub igeggjakdgj: i64,

    #[serde(rename = "IIOGPJDCBDA")]
    pub iiogpjdcbda: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "groupIdList")]
    pub group_id_list: Vec<i64>,

    #[serde(rename = "HNCOOBKCNIP")]
    pub hncoobkcnip: Vec<f64>,

    #[serde(rename = "KAFDEEFKGIG")]
    pub kafdeefkgig: f64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "ACGNPCNCHFG")]
    pub acgnpcnchfg: Acgnpcnchfg,

    #[serde(rename = "OEFMGDDBHHB")]
    pub oefmgddbhhb: String,

    #[serde(rename = "DGMIKEKBLPK")]
    pub dgmikekblpk: i64,

    #[serde(rename = "LOGJFNIGHBM")]
    pub logjfnighbm: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Acgnpcnchfg {
    #[serde(rename = "PRESENT_TYPE_NORMAL")]
    PresentTypeNormal,

    #[serde(rename = "PRESENT_TYPE_SPECIAL")]
    PresentTypeSpecial,
}

pub fn load() -> Result<ActivityVintagePresentExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityVintagePresentExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
