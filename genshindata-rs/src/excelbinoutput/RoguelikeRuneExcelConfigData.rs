// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type RoguelikeRuneExcelConfigData = Vec<RoguelikeRuneExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoguelikeRuneExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "IAJPFHLLCLD")]
    pub iajpfhllcld: bool,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "GCCBPAHOPCC")]
    pub gccbpahopcc: String,

    #[serde(rename = "PANCKHMOBHK")]
    pub panckhmobhk: String,

    #[serde(rename = "EFOJCEMCPBI")]
    pub efojcemcpbi: String,

    #[serde(rename = "MIKNOKJPIPG")]
    pub miknokjpipg: String,

    #[serde(rename = "MIDMNNOBHPJ")]
    pub midmnnobhpj: i64,

    #[serde(rename = "elementType")]
    pub element_type: i64,

    #[serde(rename = "abilityName")]
    pub ability_name: String,
}

pub fn load() -> Result<RoguelikeRuneExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "RoguelikeRuneExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
