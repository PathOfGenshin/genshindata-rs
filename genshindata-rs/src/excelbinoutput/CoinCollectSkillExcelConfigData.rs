// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type CoinCollectSkillExcelConfigData = Vec<CoinCollectSkillExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinCollectSkillExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "abilityName")]
    pub ability_name: Vec<String>,

    #[serde(rename = "AKGINPADGCH")]
    pub akginpadgch: Option<bool>,

    #[serde(rename = "LKIHNDDKPMF")]
    pub lkihnddkpmf: String,

    #[serde(rename = "HCBFKDMDJML")]
    pub hcbfkdmdjml: i64,

    #[serde(rename = "DKOLHEPIKIP")]
    pub dkolhepikip: i64,

    #[serde(rename = "GGIGMGKHGFD")]
    pub ggigmgkhgfd: Option<bool>,
}

pub fn load() -> Result<CoinCollectSkillExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "CoinCollectSkillExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
