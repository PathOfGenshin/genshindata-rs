/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type UgcOfficialLevelExcelConfigData = Vec<UgcOfficialLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct UgcOfficialLevelExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub plpidbllbef: i64,
    pub ldiadlienfh: i64,
    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,
    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,
    #[serde(rename = "previewPath")]
    pub preview_path: String,
    pub fonklmebajj: Vec<i64>,
    pub kpolfopbgjp: i64,
    pub okedgeoghep: f64,
}
