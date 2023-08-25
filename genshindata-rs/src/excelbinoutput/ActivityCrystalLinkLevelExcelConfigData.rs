/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityCrystalLinkLevelExcelConfigData = Vec<ActivityCrystalLinkLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityCrystalLinkLevelExcelConfigDatum {
    pub level_id: i64,
    pub schedule_id: i64,
    pub open_day: i64,
    pub dungeon_id: i64,
    #[serde(rename = "OKAOMEGBHBI")]
    pub okaomegbhbi: Vec<i64>,
    pub watcher_id_list: Vec<i64>,
    #[serde(rename = "JMIANMNMJBE")]
    pub jmianmnmjbe: Vec<i64>,
    #[serde(rename = "PFDACLLNCMN")]
    pub pfdacllncmn: Vec<i64>,
    pub level_title_text_map_hash: i64,
    pub level_desc_text_map_hash: i64,
    #[serde(rename = "KPGDHGGMKJL")]
    pub kpgdhggmkjl: Vec<Kpgdhggmkjl>,
    #[serde(rename = "NEHACBLEGHD")]
    pub nehacbleghd: Vec<Nehacbleghd>,
    pub score_level_list: Vec<i64>,
    #[serde(rename = "JLDDMAAAIFA")]
    pub jlddmaaaifa: i64,
    #[serde(rename = "BMMDJKMKFCE")]
    pub bmmdjkmkfce: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Kpgdhggmkjl {
    pub oopacmmkejm: Vec<String>,
    pub kddndnpdafk: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Nehacbleghd {
    pub oopacmmkejm: Vec<i64>,
    pub kddndnpdafk: Vec<i64>,
}
