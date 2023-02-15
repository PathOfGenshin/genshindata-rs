// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LunaRiteSearchingExcelConfigData = Vec<LunaRiteSearchingExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LunaRiteSearchingExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "EINIPCMDHJJ")]
    pub einipcmdhjj: String,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "GDABEMKBKIG")]
    pub gdabemkbkig: i64,

    #[serde(rename = "IHHONBFLPJM")]
    pub ihhonbflpjm: Vec<i64>,

    #[serde(rename = "regionCenter")]
    pub region_center: Vec<f64>,

    #[serde(rename = "NADEOMEPDJA")]
    pub nadeomepdja: i64,

    #[serde(rename = "regionRadius")]
    pub region_radius: f64,

    #[serde(rename = "PFOCNLJOAJI")]
    pub pfocnljoaji: i64,

    #[serde(rename = "AGBHIKJGJHL")]
    pub agbhikjgjhl: i64,

    #[serde(rename = "PJCINKMHADA")]
    pub pjcinkmhada: i64,

    #[serde(rename = "DPECKLHCNKD")]
    pub dpecklhcnkd: i64,
}

pub fn load() -> Result<LunaRiteSearchingExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "LunaRiteSearchingExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
