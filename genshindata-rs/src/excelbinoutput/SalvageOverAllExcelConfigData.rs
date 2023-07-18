/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type SalvageOverAllExcelConfigData = Vec<SalvageOverAllExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SalvageOverAllExcelConfigDatum {
    pub id: i64,
    pub activity_id: i64,
    #[serde(rename = "JHCCIJGMNHO")]
    pub jhccijgmnho: i64,
    pub pre_quest_id: i64,
    pub guide_quest_id: i64,
    pub name_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub region_center: Vec<i64>,
    pub region_radius: i64,
    #[serde(rename = "BLEPLONKDNP")]
    pub bleplonkdnp: i64,
    pub reminder_id: i64,
    #[serde(rename = "AFEJNJNLDCB")]
    pub afejnjnldcb: i64,
    pub reward_preview_id: i64,
    #[serde(rename = "LLDGEHMOEMA")]
    pub lldgehmoema: i64,
    #[serde(rename = "NJBELMMEIAK")]
    pub njbelmmeiak: i64,
}
