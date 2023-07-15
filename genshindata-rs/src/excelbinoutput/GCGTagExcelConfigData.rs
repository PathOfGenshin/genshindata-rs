/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgTagExcelConfigData = Vec<GcgTagExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GcgTagExcelConfigDatum {
    #[serde(rename = "type")]
    pub gcg_tag_excel_config_datum_type: String,
    pub eieibcdfepj: Vec<Eieibcdfepj>,
    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,
    pub giabelgokde: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Eieibcdfepj {
    #[serde(rename = "GCG_TAG_IDENTIFIER_ASSIST")]
    GcgTagIdentifierAssist,
    #[serde(rename = "GCG_TAG_IDENTIFIER_CHAR")]
    GcgTagIdentifierChar,
    #[serde(rename = "GCG_TAG_IDENTIFIER_ELEMENT")]
    GcgTagIdentifierElement,
    #[serde(rename = "GCG_TAG_IDENTIFIER_EVENT")]
    GcgTagIdentifierEvent,
    #[serde(rename = "GCG_TAG_IDENTIFIER_MODIFY")]
    GcgTagIdentifierModify,
    #[serde(rename = "GCG_TAG_IDENTIFIER_NONE")]
    GcgTagIdentifierNone,
    #[serde(rename = "GCG_TAG_IDENTIFIER_WEAPON")]
    GcgTagIdentifierWeapon,
}
