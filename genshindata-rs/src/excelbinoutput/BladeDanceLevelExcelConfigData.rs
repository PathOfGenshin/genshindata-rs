/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BladeDanceLevelExcelConfigData = Vec<BladeDanceLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct BladeDanceLevelExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub ecchphppcff: i64,
    pub kjdoghpgabm: i64,
    pub amhicgnnmka: i64,
    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,
    pub amjkmcfcldo: i64,
    #[serde(rename = "galleryId")]
    pub gallery_id: i64,
    #[serde(rename = "groupId")]
    pub group_id: i64,
    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,
    #[serde(rename = "buffList")]
    pub buff_list: Vec<i64>,
    pub abjaokamdpg: i64,
    pub kbpmhjogmha: i64,
}
