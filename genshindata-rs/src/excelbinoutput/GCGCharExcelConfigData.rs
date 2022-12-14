// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GcgCharExcelConfigData = Vec<GcgCharExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GcgCharExcelConfigDatum {
    #[serde(rename = "hp")]
    pub hp: i64,

    #[serde(rename = "BPHBKAGLFCE")]
    pub bphbkaglfce: i64,

    #[serde(rename = "HLKMHIIIFHA")]
    pub hlkmhiiifha: i64,

    #[serde(rename = "IAPINBOEJCO")]
    pub iapinboejco: String,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "cardType")]
    pub card_type: CardType,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "LDKBFGDBOKD")]
    pub ldkbfgdbokd: Vec<String>,

    #[serde(rename = "skillList")]
    pub skill_list: Vec<i64>,

    #[serde(rename = "PICLELHGDBJ")]
    pub piclelhgdbj: Option<bool>,

    #[serde(rename = "FANGCKEJDOL")]
    pub fangckejdol: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CardType {
    #[serde(rename = "GCG_CARD_CHARACTER")]
    GcgCardCharacter,
}

pub fn load() -> Result<GcgCharExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GCGCharExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
