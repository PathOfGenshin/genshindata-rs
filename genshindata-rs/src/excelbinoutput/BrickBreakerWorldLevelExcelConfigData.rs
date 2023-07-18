/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BrickBreakerWorldLevelExcelConfigData = Vec<BrickBreakerWorldLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct BrickBreakerWorldLevelExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub hfghoggplaf: Option<i64>,
    #[serde(rename = "draftId")]
    pub draft_id: Option<i64>,
    #[serde(rename = "limitTime")]
    pub limit_time: i64,
    pub obhfnppkbfc: i64,
    #[serde(rename = "sceneId")]
    pub scene_id: i64,
    #[serde(rename = "groupId")]
    pub group_id: i64,
    #[serde(rename = "transportPointList")]
    pub transport_point_list: Vec<i64>,
    #[serde(rename = "durationList")]
    pub duration_list: Vec<i64>,
    #[serde(rename = "galleryId")]
    pub gallery_id: i64,
    #[serde(rename = "levelTitleTextMapHash")]
    pub level_title_text_map_hash: i64,
    pub gnccbmdajeh: i64,
    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,
    pub dojcogajmoe: f64,
    pub ladmpjogfbn: Vec<i64>,
    pub hhgobabckgc: Vec<Hhgobabckgc>,
    pub eopjmaidfgm: Vec<i64>,
    pub bebgejeapie: Option<bool>,
    pub hecdjdodobc: Option<i64>,
    pub babpggfffop: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Hhgobabckgc {
    Fire,
    None,
    Water,
}
