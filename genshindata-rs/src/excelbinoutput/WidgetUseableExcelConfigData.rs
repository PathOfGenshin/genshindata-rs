/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type WidgetUseableExcelConfigData = Vec<WidgetUseableExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetUseableExcelConfigDatum {
    #[serde(rename = "materialID")]
    pub material_id: i64,
    pub can_use_in_other_world: Option<bool>,
    pub forbidden_scene_id_list: Vec<i64>,
    #[serde(rename = "PEHHANFPAJO")]
    pub pehhanfpajo: Option<bool>,
    pub forbidden_dungeon_type_list: Vec<i64>,
    pub forbidden_dungeon_play_type_list: Vec<i64>,
    #[serde(rename = "PNECGNFCHIL")]
    pub pnecgnfchil: Vec<i64>,
    #[serde(rename = "HDOGABBAPID")]
    pub hdogabbapid: Option<bool>,
    pub can_use_in_dungeon: Option<bool>,
    pub can_use_in_homeworld: Option<bool>,
    #[serde(rename = "PBPDKKIJBKL")]
    pub pbpdkkijbkl: Option<bool>,
    pub can_use_in_room: Option<bool>,
    pub can_use_in_limit_region: Option<bool>,
    pub vehicle_limit: Option<String>,
}
