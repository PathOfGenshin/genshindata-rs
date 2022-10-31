// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;
use std::collections::HashMap;

pub type GatherBundleExcelConfigData = Vec<GatherBundleExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GatherBundleExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "bundleName")]
    pub bundle_name: String,

    #[serde(rename = "baseGadgetId")]
    pub base_gadget_id: i64,

    #[serde(rename = "points")]
    pub points: Vec<HashMap<String, f64>>,
}

pub fn load() -> Result<GatherBundleExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "GatherBundleExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}