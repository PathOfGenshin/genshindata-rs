/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FungusFighterV3EnemyExcelConfigData = Vec<FungusFighterV3EnemyExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FungusFighterV3EnemyExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "sortID")]
    pub sort_id: i64,
    pub unlock_level: i64,
    pub icon_path: String,
    #[serde(rename = "EDADMEAFGPA")]
    pub edadmeafgpa: String,
    pub name_text_map_hash: i64,
    pub description_text_map_hash: i64,
    #[serde(rename = "ODPOIIPDDCK")]
    pub odpoiipddck: Vec<i64>,
    #[serde(rename = "KEPJHJIFJPH")]
    pub kepjhjifjph: Option<bool>,
}