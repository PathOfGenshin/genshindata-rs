// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GadgetGuestExcelConfigData = Vec<GadgetGuestExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GadgetGuestExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "showType")]
    pub show_type: Option<ShowType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ShowType {
    #[serde(rename = "GRAP")]
    Grap,
}

pub fn load() -> Result<GadgetGuestExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GadgetGuestExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
