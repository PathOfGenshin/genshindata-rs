/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GravenInnocenceBossDataExcelConfigData = Vec<GravenInnocenceBossDataExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GravenInnocenceBossDataExcelConfigDatum {
    pub id: i64,
    pub boss_title_text_map_hash: i64,
    pub watcher_list: Vec<i64>,
    #[serde(rename = "adventureID")]
    pub adventure_id: i64,
}
