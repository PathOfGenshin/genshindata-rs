/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ShuffleBoardChequerExcelConfigData = Vec<ShuffleBoardChequerExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ShuffleBoardChequerExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "gadgetId")]
    pub gadget_id: i64,
    pub jjkpekgpipp: String,
    pub jipngnegjod: String,
    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,
    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
    pub inabgmhbika: i64,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
    pub glkmcfjodgl: String,
    pub ffogaghmjhj: Option<bool>,
}
