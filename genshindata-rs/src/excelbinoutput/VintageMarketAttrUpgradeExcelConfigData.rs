// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type VintageMarketAttrUpgradeExcelConfigData = Vec<VintageMarketAttrUpgradeExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VintageMarketAttrUpgradeExcelConfigDatum {
    #[serde(rename = "BDHOMHLAAJF")]
    pub bdhomhlaajf: Vec<Bdhomhlaajf>,

    #[serde(rename = "id")]
    pub id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bdhomhlaajf {
    #[serde(rename = "NPNLKGOIGIG")]
    pub npnlkgoigig: Vec<i64>,
}

pub fn load() -> Result<VintageMarketAttrUpgradeExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "VintageMarketAttrUpgradeExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
