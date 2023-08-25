/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AkaFesPotionLevelExcelConfigData = Vec<AkaFesPotionLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AkaFesPotionLevelExcelConfigDatum {
    pub id: i64,
    pub open_day: i64,
    #[serde(rename = "AGLIJDNGAPK")]
    pub aglijdngapk: i64,
    pub level_title_text_map_hash: i64,
    pub level_desc_text_map_hash: i64,
    pub watcher_list: Vec<i64>,
    pub dungeon_id: i64,
    pub gallery_id: i64,
    #[serde(rename = "EBJIPEHADMF")]
    pub ebjipehadmf: Vec<i64>,
    pub monster_id_list: Vec<i64>,
    #[serde(rename = "CIPGHJCDKJP")]
    pub cipghjcdkjp: Vec<i64>,
    #[serde(rename = "HCBCLBLAJDP")]
    pub hcbclblajdp: Vec<i64>,
}
