// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GcgRuleTextDetailExcelConfigData = Vec<GcgRuleTextDetailExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GcgRuleTextDetailExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "IFJNLHHKLME")]
    pub ifjnlhhklme: Ifjnlhhklme,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "contentTextMapHash")]
    pub content_text_map_hash: i64,

    #[serde(rename = "LJFOKNADHHA")]
    pub ljfoknadhha: Option<bool>,

    #[serde(rename = "BPFBMDOCDFK")]
    pub bpfbmdocdfk: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Ifjnlhhklme {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_Gcg_InSide_01")]
    UiGcgInSide01,

    #[serde(rename = "UI_Gcg_InSide_11")]
    UiGcgInSide11,

    #[serde(rename = "UI_Gcg_InSide_12")]
    UiGcgInSide12,

    #[serde(rename = "UI_Gcg_InSide_13")]
    UiGcgInSide13,

    #[serde(rename = "UI_Gcg_InSide_14")]
    UiGcgInSide14,
}

pub fn load() -> Result<GcgRuleTextDetailExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GCGRuleTextDetailExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
