/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AvatarCostumeExcelConfigData = Vec<AvatarCostumeExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarCostumeExcelConfigDatum {
    pub skin_id: i64,
    #[serde(rename = "indexID")]
    pub index_id: Option<i64>,
    pub name_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub item_id: Option<i64>,
    pub character_id: i64,
    pub json_name: String,
    pub prefab_path_hash: Option<f64>,
    pub prefab_remote_path_hash: Option<f64>,
    pub prefab_npc_path_hash: Option<f64>,
    pub prefab_manekin_path_hash: Option<f64>,
    pub quality: Option<i64>,
    pub front_icon_name: String,
    pub side_icon_name: String,
    pub image_name_hash: Option<f64>,
    pub is_default_unlock: Option<bool>,
    pub hide: Option<bool>,
    pub animator_config_path_hash: Option<f64>,
    pub controller_path_hash: Option<f64>,
    pub controller_remote_path_hash: Option<f64>,
    pub is_default: Option<bool>,
    pub domestic_hide_in_art_preview: Option<bool>,
}
