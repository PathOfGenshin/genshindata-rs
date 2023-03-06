// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MichiaeOfferingDataExcelConfigData = Vec<MichiaeOfferingDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MichiaeOfferingDataExcelConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "DBABBDKOAAM")]
    pub dbabbdkoaam: i64,

    #[serde(rename = "DEINFMBCIJH")]
    pub deinfmbcijh: Vec<String>,

    #[serde(rename = "AJOFBINPIEB")]
    pub ajofbinpieb: i64,

    #[serde(rename = "CENCEKBMMPL")]
    pub cencekbmmpl: Vec<String>,

    #[serde(rename = "level")]
    pub level: Option<i64>,
}

pub fn load() -> Result<MichiaeOfferingDataExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MichiaeOfferingDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
