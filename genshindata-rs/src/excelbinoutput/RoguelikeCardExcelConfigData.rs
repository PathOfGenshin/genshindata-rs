// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type RoguelikeCardExcelConfigData = Vec<RoguelikeCardExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoguelikeCardExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "sortOrder")]
    pub sort_order: i64,

    #[serde(rename = "type")]
    pub roguelike_card_excel_config_datum_type: Type,

    #[serde(rename = "EOHCDKMPDNN")]
    pub eohcdkmpdnn: Vec<i64>,

    #[serde(rename = "CKNAMPIDGOD")]
    pub cknampidgod: Vec<String>,

    #[serde(rename = "MOCCNJPNFCP")]
    pub moccnjpnfcp: Moccnjpnfcp,

    #[serde(rename = "FIJJOLEOOMN")]
    pub fijjoleoomn: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "KGJPFMIOECN")]
    pub kgjpfmioecn: i64,

    #[serde(rename = "descParamList")]
    pub desc_param_list: Vec<f64>,

    #[serde(rename = "IPHDOCKEMDE")]
    pub iphdockemde: Vec<bool>,

    #[serde(rename = "MEOJNEKJIIL")]
    pub meojnekjiil: Vec<f64>,

    #[serde(rename = "MHBKMJMMBGJ")]
    pub mhbkmjmmbgj: Option<Mhbkmjmmbgj>,

    #[serde(rename = "MEJPPEIAEAA")]
    pub mejppeiaeaa: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Moccnjpnfcp {
    #[serde(rename = "effectType")]
    pub effect_type: String,

    #[serde(rename = "PIOMJPCNDKA")]
    pub piomjpcndka: String,

    #[serde(rename = "AHIDGHIMNBK")]
    pub ahidghimnbk: String,

    #[serde(rename = "MCNFEOJIGKK")]
    pub mcnfeojigkk: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Mhbkmjmmbgj {
    #[serde(rename = "ROGUELIKE_CARD_LABEL_EQUIPMENT")]
    RoguelikeCardLabelEquipment,

    #[serde(rename = "ROGUELIKE_CARD_LABEL_LEVEL")]
    RoguelikeCardLabelLevel,

    #[serde(rename = "ROGUELIKE_CARD_LABEL_RUNE")]
    RoguelikeCardLabelRune,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ROGUELIKE_CARD_TPYE_R")]
    RoguelikeCardTpyeR,

    #[serde(rename = "ROGUELIKE_CARD_TPYE_SR")]
    RoguelikeCardTpyeSr,

    #[serde(rename = "ROGUELIKE_CARD_TPYE_SSR")]
    RoguelikeCardTpyeSsr,
}

pub fn load() -> Result<RoguelikeCardExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "RoguelikeCardExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
