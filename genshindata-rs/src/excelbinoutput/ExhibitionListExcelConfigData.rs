// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ExhibitionListExcelConfigData = Vec<ExhibitionListExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExhibitionListExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "FLFFNIIDFNB")]
    pub flffniidfnb: i64,

    #[serde(rename = "JCBMNNCPECH")]
    pub jcbmnncpech: i64,

    #[serde(rename = "NGLJJIPEFPK")]
    pub ngljjipefpk: i64,

    #[serde(rename = "GKBEMNNIHLK")]
    pub gkbemnnihlk: i64,

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
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ExhibitionListExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
