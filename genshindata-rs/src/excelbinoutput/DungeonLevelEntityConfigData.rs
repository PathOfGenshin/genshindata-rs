/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type DungeonLevelEntityConfigData = Vec<DungeonLevelEntityConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DungeonLevelEntityConfigDatum {
    pub client_id: i64,
    pub id: i64,
    pub show: Option<bool>,
    pub level_config_name: String,
    pub desc_text_map_hash: i64,
    #[serde(rename = "MOOJOOMKILO")]
    pub moojoomkilo: i64,
}
