/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityCrystalLinkLevelExcelConfigData = Vec<ActivityCrystalLinkLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityCrystalLinkLevelExcelConfigDatum {
    pub level_id: i64,
    pub schedule_id: i64,
    pub open_day: i64,
    pub dungeon_id: i64,
    pub trial_avatar_list: Vec<i64>,
    pub watcher_id_list: Vec<i64>,
    pub cond_buff_id_list: Vec<i64>,
    pub effect_buff_id_list: Vec<i64>,
    pub level_title_text_map_hash: i64,
    pub level_desc_text_map_hash: i64,
    pub monster_config_array: Vec<MonsterConfigArray>,
    pub main_monster_config_array: Vec<MainMonsterConfigArray>,
    pub score_level_list: Vec<i64>,
    #[serde(rename = "condCD")]
    pub cond_cd: i64,
    pub eff_last_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainMonsterConfigArray {
    pub boss: Vec<i64>,
    pub monster: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterConfigArray {
    pub boss: Vec<String>,
    pub monster: Vec<String>,
}
