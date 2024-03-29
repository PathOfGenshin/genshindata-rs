/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LoadingCustomExcelConfigData = Vec<LoadingCustomExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LoadingCustomExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "dungeonID")]
    pub dungeon_id: Option<i64>,
    #[serde(rename = "perfabPath")]
    pub perfab_path: String,
    pub kcijdbedbek: Option<i64>,
}
