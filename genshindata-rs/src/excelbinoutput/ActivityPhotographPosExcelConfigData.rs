/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityPhotographPosExcelConfigData = Vec<ActivityPhotographPosExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityPhotographPosExcelConfigDatum {
    pub id: i64,
    pub group_link_id: i64,
    pub pos_title_text_map_hash: i64,
    pub require_title_in_activity_page_text_map_hash: i64,
    pub require_desc_in_activity_page_text_map_hash: i64,
    pub npc_invite_desc_text_map_hash: i64,
    pub npc_comment_desc_text_map_hash: i64,
    pub pic_small: String,
    pub pic_big: String,
    #[serde(rename = "photoCheckRootID")]
    pub photo_check_root_id: i64,
    pub root_node_desc_text_map_hash: i64,
    #[serde(rename = "photoCheckSubNodeID")]
    pub photo_check_sub_node_id: Vec<i64>,
    pub photo_check_sub_node_desc: Vec<i64>,
    pub open_day: i64,
    pub watcher_id: i64,
    pub gallery_id: i64,
    #[serde(rename = "redPointID")]
    pub red_point_id: i64,
    #[serde(rename = "KFJMCPIAPPE")]
    pub kfjmcpiappe: i64,
    #[serde(rename = "MOBPCKLKBAK")]
    pub mobpcklkbak: i64,
    #[serde(rename = "FOAPFCOJCAN")]
    pub foapfcojcan: i64,
    #[serde(rename = "KHNJIACMHIC")]
    pub khnjiacmhic: Vec<String>,
    #[serde(rename = "INCGNECKJKO")]
    pub incgneckjko: Vec<String>,
    #[serde(rename = "sceneID")]
    pub scene_id: i64,
}
