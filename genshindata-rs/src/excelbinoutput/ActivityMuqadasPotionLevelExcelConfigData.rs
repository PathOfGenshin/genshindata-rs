/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityMuqadasPotionLevelExcelConfigData = Vec<ActivityMuqadasPotionLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMuqadasPotionLevelExcelConfigDatum {
    pub level_id: i64,
    pub open_day_index: i64,
    pub dungeon_id: i64,
    pub gallery_id: i64,
    pub trial_avatar_list: Vec<i64>,
    pub normal_skill_name: Vec<NormalSkillName>,
    pub dungeon_name_text_map_hash: i64,
    pub dungeon_desc_text_map_hash: i64,
    pub skill_desc_text_map_hash: i64,
    pub in_level_skill_desc_com_text_map_hash: String,
    pub in_level_skill_desc_ad_text_map_hash: String,
    pub watcher_list: Vec<i64>,
    pub skill_threshold: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NormalSkillName {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "6058,6021,6046,6041")]
    The6058602160466041,
}
