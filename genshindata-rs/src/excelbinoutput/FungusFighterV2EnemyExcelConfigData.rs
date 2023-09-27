/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FungusFighterV2EnemyExcelConfigData = Vec<FungusFighterV2EnemyExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FungusFighterV2EnemyExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "sortID")]
    pub sort_id: i64,
    #[serde(rename = "unlockLevel")]
    pub unlock_level: i64,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
    pub palokilpdci: String,
    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,
    #[serde(rename = "descriptionTextMapHash")]
    pub description_text_map_hash: i64,
    pub ndfcdannbkn: i64,
    pub fpidikhjala: Fpidikhjala,
    pub bmombhkoldi: Vec<i64>,
    pub hjgjfodeojm: Option<i64>,
    pub mdabcenafgf: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Fpidikhjala {
    #[serde(rename = "ART/UI/Atlas/BuffElement/UI_Tips_Item_Warning")]
    ArtUiAtlasBuffElementUiTipsItemWarning,
    #[serde(rename = "")]
    Empty,
}
