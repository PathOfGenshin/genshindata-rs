/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type CustomLevelDungeonConfigData = Vec<CustomLevelDungeonConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomLevelDungeonConfigDatum {
    #[serde(rename = "dungeonID")]
    pub dungeon_id: i64,
    pub order: i64,
    pub json_path: String,
    pub map_prefab_path: String,
    pub map_scene_pic_hash: Option<f64>,
    pub data_pic_hash: Option<f64>,
    pub room_num: i64,
    pub room_name_format_text_map_hash: i64,
    pub corridors: String,
    pub count_brick_list: Vec<i64>,
}
