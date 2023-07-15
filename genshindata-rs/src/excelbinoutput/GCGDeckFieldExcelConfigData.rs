/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgDeckFieldExcelConfigData = Vec<GcgDeckFieldExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GcgDeckFieldExcelConfigDatum {
    #[serde(rename = "itemId")]
    pub item_id: i64,
    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,
    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
    pub plbjnadgdef: i64,
    #[serde(rename = "order")]
    pub order: i64,
    pub adicakabndp: f64,
    pub bnkppdpnafh: String,
    pub hhnbgppijbi: i64,
    pub ddjipeopgeb: i64,
    pub hggdjpdbncc: Vec<String>,
    pub hncbgdmimjm: Vec<String>,
    pub ediihmlphha: Vec<String>,
    pub iabbfdpejdj: String,
    pub pdgelgajpjk: String,
    pub poondajgjoe: String,
    #[serde(rename = "id")]
    pub id: Option<i64>,
}
