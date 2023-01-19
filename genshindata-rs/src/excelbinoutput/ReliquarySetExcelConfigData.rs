// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ReliquarySetExcelConfigData = Vec<ReliquarySetExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReliquarySetExcelConfigDatum {
    #[serde(rename = "setId")]
    pub set_id: i64,

    #[serde(rename = "setIcon")]
    pub set_icon: String,

    #[serde(rename = "setNeedNum")]
    pub set_need_num: Vec<i64>,

    #[serde(rename = "EquipAffixId")]
    pub equip_affix_id: Option<i64>,

    #[serde(rename = "containsList")]
    pub contains_list: Vec<i64>,

    #[serde(rename = "LONEOHOJGNK")]
    pub loneohojgnk: Option<i64>,

    #[serde(rename = "JAAHBKDEOMM")]
    pub jaahbkdeomm: Vec<i64>,

    #[serde(rename = "textList")]
    pub text_list: Vec<i64>,

    #[serde(rename = "DisableFilter")]
    pub disable_filter: Option<i64>,
}

pub fn load() -> Result<ReliquarySetExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ReliquarySetExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
