/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type WorldLevelExcelConfigData = Vec<WorldLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldLevelExcelConfigDatum {
    pub level: i64,
    pub monster_level: i64,
}
