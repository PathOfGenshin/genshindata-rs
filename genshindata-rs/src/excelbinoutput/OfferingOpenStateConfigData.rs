// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type OfferingOpenStateConfigData = Vec<OfferingOpenStateConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferingOpenStateConfigDatum {
    #[serde(rename = "offeringId")]
    pub offering_id: i64,

    #[serde(rename = "openState")]
    pub open_state: String,

    #[serde(rename = "itemLimit")]
    pub item_limit: String,

    #[serde(rename = "EMBHAECKKMB")]
    pub embhaeckkmb: String,

    #[serde(rename = "IEEICFNHHJC")]
    pub ieeicfnhhjc: Option<bool>,

    #[serde(rename = "ALGAAGIPMFE")]
    pub algaagipmfe: Option<bool>,

    #[serde(rename = "activityId")]
    pub activity_id: Option<i64>,
}

pub fn load() -> Result<OfferingOpenStateConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "OfferingOpenStateConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
