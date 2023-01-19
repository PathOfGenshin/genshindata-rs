// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type BartenderTaskOrderExcelConfigData = Vec<BartenderTaskOrderExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BartenderTaskOrderExcelConfigDatum {
    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "KDLCJAEOGLB")]
    pub kdlcjaeoglb: Vec<i64>,

    #[serde(rename = "DKMMHCJKAIP")]
    pub dkmmhcjkaip: i64,

    #[serde(rename = "LGMPJPICPMP")]
    pub lgmpjpicpmp: i64,

    #[serde(rename = "FAKKOCMJBFD")]
    pub fakkocmjbfd: i64,

    #[serde(rename = "JJFFNDFBIEI")]
    pub jjffndfbiei: Option<i64>,

    #[serde(rename = "DEOBAMPPPCL")]
    pub deobampppcl: Option<bool>,
}

pub fn load() -> Result<BartenderTaskOrderExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "BartenderTaskOrderExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
