// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type CatalogExcelConfigData = Vec<CatalogExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "type")]
    pub catalog_excel_config_datum_type: String,

    #[serde(rename = "OFCDLOBEHGK")]
    pub ofcdlobehgk: Vec<Vec<i64>>,

    #[serde(rename = "MLGLNLPMCCD")]
    pub mlglnlpmccd: Vec<Mlglnlpmccd>,

    #[serde(rename = "DIFAPPALAIL")]
    pub difappalail: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mlglnlpmccd {
    #[serde(rename = "HKOJACADDNH")]
    pub hkojacaddnh: Option<i64>,

    #[serde(rename = "JMHPGKOPEBM")]
    pub jmhpgkopebm: Option<i64>,
}

pub fn load() -> Result<CatalogExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "CatalogExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
