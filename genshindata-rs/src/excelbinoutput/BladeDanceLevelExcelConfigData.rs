/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BladeDanceLevelExcelConfigData = Vec<BladeDanceLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BladeDanceLevelExcelConfigDatum {
    pub id: i64,
    pub stage: i64,
    #[serde(rename = "EBHBCBBHDEF")]
    pub ebhbcbbhdef: i64,
    #[serde(rename = "ELGABOLOGIO")]
    pub elgabologio: i64,
    pub dungeon_id: i64,
    #[serde(rename = "OGLGFAONDOH")]
    pub oglgfaondoh: i64,
    pub gallery_id: i64,
    pub group_id: i64,
    pub watcher_list: Vec<i64>,
    pub buff_list: Vec<i64>,
    #[serde(rename = "NMBPKICMPEH")]
    pub nmbpkicmpeh: i64,
    #[serde(rename = "KOGPEGEEBBD")]
    pub kogpegeebbd: i64,
}
