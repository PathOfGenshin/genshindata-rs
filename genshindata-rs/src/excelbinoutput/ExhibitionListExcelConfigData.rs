// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ExhibitionListExcelConfigData = Vec<ExhibitionListExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExhibitionListExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "IHIINMEOCNP")]
    pub ihiinmeocnp: i64,

    #[serde(rename = "KPGFELNHDPC")]
    pub kpgfelnhdpc: i64,

    #[serde(rename = "KKMENFIONKG")]
    pub kkmenfionkg: i64,

    #[serde(rename = "MAAJIGJPDMB")]
    pub maajigjpdmb: i64,

    #[serde(rename = "displayType")]
    pub display_type: DisplayType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DisplayType {
    #[serde(rename = "EXHIBITION_DISPLAY_TYPE_INT")]
    ExhibitionDisplayTypeInt,

    #[serde(rename = "EXHIBITION_DISPLAY_TYPE_INT_CHARACTER")]
    ExhibitionDisplayTypeIntCharacter,

    #[serde(rename = "EXHIBITION_DISPLAY_TYPE_TIME_MINSEC")]
    ExhibitionDisplayTypeTimeMinsec,
}

pub fn load() -> Result<ExhibitionListExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ExhibitionListExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
