/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LevelTagResetExcelConfigData = Vec<LevelTagResetExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LevelTagResetExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,
    pub kgkmkcddbfo: Vec<i64>,
}
