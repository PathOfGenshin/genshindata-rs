// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BartenderTaskOrderExcelConfigData = Vec<BartenderTaskOrderExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BartenderTaskOrderExcelConfigDatum {
    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "JOGMCADANKP")]
    pub jogmcadankp: Vec<i64>,

    #[serde(rename = "PFGMBOEDHOM")]
    pub pfgmboedhom: i64,

    #[serde(rename = "JCBGBACIGEA")]
    pub jcbgbacigea: i64,

    #[serde(rename = "KLNEAHJBANO")]
    pub klneahjbano: i64,

    #[serde(rename = "EBOIGKIHFDO")]
    pub eboigkihfdo: Option<i64>,

    #[serde(rename = "GLFCMKMJJDJ")]
    pub glfcmkmjjdj: Option<bool>,
}

pub fn load() -> Result<BartenderTaskOrderExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "BartenderTaskOrderExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
