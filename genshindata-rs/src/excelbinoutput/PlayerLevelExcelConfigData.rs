/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type PlayerLevelExcelConfigData = Vec<PlayerLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLevelExcelConfigDatum {
    pub level: i64,
    pub exp: Option<i64>,
    pub unlock_desc_text_map_hash: i64,
    pub reward_id: Option<i64>,
    pub unlock_world_level: Option<i64>,
    pub expedition_limit_add: Option<i64>,
}
