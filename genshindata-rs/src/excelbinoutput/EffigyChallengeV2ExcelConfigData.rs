// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type EffigyChallengeV2ExcelConfigData = Vec<EffigyChallengeV2ExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct EffigyChallengeV2ExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "dayIndex")]
    pub day_index: i64,

    #[serde(rename = "GJOHIBCENMI")]
    pub gjohibcenmi: i64,

    #[serde(rename = "LDOGIIJLGFL")]
    pub ldogiijlgfl: Vec<i64>,

    #[serde(rename = "FCMDFCPIBLC")]
    pub fcmdfcpiblc: Vec<i64>,

    #[serde(rename = "OKPBHHENMAH")]
    pub okpbhhenmah: Vec<i64>,

    #[serde(rename = "OFOMPLGPOKK")]
    pub ofomplgpokk: String,

    #[serde(rename = "MLHDIOICGIJ")]
    pub mlhdioicgij: String,

    #[serde(rename = "HLPNPPGAONA")]
    pub hlpnppgaona: i64,

    #[serde(rename = "DNENLPONMGB")]
    pub dnenlponmgb: Vec<i64>,

    #[serde(rename = "GLBIJOPHJKN")]
    pub glbijophjkn: Vec<i64>,

    #[serde(rename = "EAMODOGEMPB")]
    pub eamodogempb: Vec<i64>,

    #[serde(rename = "levelTitleTextMapHash")]
    pub level_title_text_map_hash: i64,

    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,

    #[serde(rename = "DCJKJOMAPOI")]
    pub dcjkjomapoi: i64,

    #[serde(rename = "FCFFFLHDFDE")]
    pub fcffflhdfde: i64,
}

pub fn load() -> Result<EffigyChallengeV2ExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "EffigyChallengeV2ExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
