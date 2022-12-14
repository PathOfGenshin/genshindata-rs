// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type BlossomChestExcelConfigData = Vec<BlossomChestExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlossomChestExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "chestGadgetId")]
    pub chest_gadget_id: i64,

    #[serde(rename = "worldResin")]
    pub world_resin: Option<i64>,

    #[serde(rename = "resin")]
    pub resin: Option<i64>,

    #[serde(rename = "refreshType")]
    pub refresh_type: String,

    #[serde(rename = "clientShowType")]
    pub client_show_type: Option<String>,
}

pub fn load() -> Result<BlossomChestExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "BlossomChestExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
