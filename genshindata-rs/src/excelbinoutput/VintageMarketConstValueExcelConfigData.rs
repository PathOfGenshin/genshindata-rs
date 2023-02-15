// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type VintageMarketConstValueExcelConfigData = Vec<VintageMarketConstValueExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VintageMarketConstValueExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "OLEPGNPCJOF")]
    pub olepgnpcjof: i64,

    #[serde(rename = "NLOCPKHHMEP")]
    pub nlocpkhhmep: i64,

    #[serde(rename = "JKMHMNDGGJP")]
    pub jkmhmndggjp: i64,

    #[serde(rename = "CILHBJCPDGA")]
    pub cilhbjcpdga: i64,

    #[serde(rename = "CEPIFIIOPFG")]
    pub cepifiiopfg: i64,

    #[serde(rename = "CNDABFCMFDB")]
    pub cndabfcmfdb: i64,

    #[serde(rename = "EJJCHFPNAGM")]
    pub ejjchfpnagm: f64,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,

    #[serde(rename = "BOGOPAMGCAG")]
    pub bogopamgcag: i64,

    #[serde(rename = "JCNGFDLHFHO")]
    pub jcngfdlhfho: i64,

    #[serde(rename = "NKILMDNPEIJ")]
    pub nkilmdnpeij: Vec<i64>,

    #[serde(rename = "EBEGJJMNMLE")]
    pub ebegjjmnmle: i64,

    #[serde(rename = "JAGBFJAAPLK")]
    pub jagbfjaaplk: i64,

    #[serde(rename = "KIICNAIJCCH")]
    pub kiicnaijcch: i64,
}

pub fn load() -> Result<VintageMarketConstValueExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "VintageMarketConstValueExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
