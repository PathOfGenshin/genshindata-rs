/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type SceneExcelConfigData = Vec<SceneExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "type")]
    pub scene_excel_config_datum_type: Type,
    pub script_data: String,
    pub override_default_profile: String,
    pub level_entity_config: String,
    pub specified_avatar_list: Vec<i64>,
    pub comment: Comment,
    #[serde(rename = "FOKGBGKNMPB")]
    pub fokgbgknmpb: Vec<f64>,
    pub dungeon_entry_point: Vec<i64>,
    pub max_specified_avatar_num: Option<i64>,
    pub ignore_nav_mesh: Option<bool>,
    pub world_scene_id: Option<i64>,
    pub mp_type: Option<String>,
    pub use_to_the_moon: Option<bool>,
    pub navmesh_mode: Option<String>,
    pub safe_point: Option<i64>,
    pub is_allow_map_mark_point: Option<bool>,
    pub is_delete_map_mark_point: Option<bool>,
    pub scene_fix_time: Option<f64>,
    pub entity_appear_sorted: Option<i64>,
    pub subtype: Option<Subtype>,
    #[serde(rename = "FHMGABPEHAJ")]
    pub fhmgabpehaj: Option<i64>,
    #[serde(rename = "ECNGICKDCAM")]
    pub ecngickdcam: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Comment {
    #[serde(rename = "正式")]
    Comment,
    #[serde(rename = "测试")]
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    #[serde(rename = "SCENE_DUNGEON")]
    SceneDungeon,
    #[serde(rename = "SCENE_HOME_ROOM")]
    SceneHomeRoom,
    #[serde(rename = "SCENE_HOME_WORLD")]
    SceneHomeWorld,
    #[serde(rename = "SCENE_ROOM")]
    SceneRoom,
    #[serde(rename = "SCENE_WORLD")]
    SceneWorld,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Subtype {
    #[serde(rename = "SCENE_SUB_TYPE_PERSISTENT_DUNGEON")]
    SceneSubTypePersistentDungeon,
    #[serde(rename = "SCENE_SUB_TYPE_UGC_PLAY")]
    SceneSubTypeUgcPlay,
}
