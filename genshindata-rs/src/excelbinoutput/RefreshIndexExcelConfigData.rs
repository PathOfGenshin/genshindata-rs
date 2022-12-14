// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type RefreshIndexExcelConfigData = Vec<RefreshIndexExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshIndexExcelConfigDatum {
    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "type")]
    pub refresh_index_excel_config_datum_type: Option<Type>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "refreshId")]
    pub refresh_id: i64,

    #[serde(rename = "rarity")]
    pub rarity: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "REFRESHINDEX_GADGET")]
    RefreshindexGadget,
}

pub fn load() -> Result<RefreshIndexExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "RefreshIndexExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
