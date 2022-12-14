// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type VintageMarketRoundExcelConfigData = Vec<VintageMarketRoundExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VintageMarketRoundExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "HCBDBMEOFGE")]
    pub hcbdbmeofge: Vec<Hcbdbmeofge>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hcbdbmeofge {
    #[serde(rename = "KOJMAFGFLHP")]
    pub kojmafgflhp: Vec<f64>,

    #[serde(rename = "GJHBMONEKOD")]
    pub gjhbmonekod: i64,

    #[serde(rename = "OIOHBCCPCFO")]
    pub oiohbccpcfo: f64,

    #[serde(rename = "OBCGKJEPMKK")]
    pub obcgkjepmkk: Option<i64>,

    #[serde(rename = "KJIHNGMNBPA")]
    pub kjihngmnbpa: f64,
}

pub fn load() -> Result<VintageMarketRoundExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "VintageMarketRoundExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
