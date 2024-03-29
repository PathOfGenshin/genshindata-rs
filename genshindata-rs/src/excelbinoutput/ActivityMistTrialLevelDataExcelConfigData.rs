/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityMistTrialLevelDataExcelConfigData = Vec<ActivityMistTrialLevelDataExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMistTrialLevelDataExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,
    pub schedule_id: i64,
    pub level_id: i64,
    pub level_title_text_map_hash: i64,
    pub level_desc_text_map_hash: i64,
    pub monster_preview_id_list: Vec<i64>,
    pub key_monster_preview_id_list: Vec<i64>,
    pub open_day: i64,
    pub challenge_mission_watcher_list: Vec<i64>,
    pub statistics_id_list: Vec<i64>,
    pub bg_icon_hash: f64,
    pub dungeon_id: i64,
    pub dungeon_factor_id_list: Vec<i64>,
    pub fail_tips: Vec<String>,
    pub trial_avatar_id_list: Vec<i64>,
    pub server_global_value_key: ServerGlobalValueKey,
    pub floor_level_list: Vec<i64>,
    pub battle_statistics_data: Vec<i64>,
    #[serde(rename = "JIEEIFDAIEK")]
    pub jieeifdaiek: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerGlobalValueKey {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "SGV_ABILITY_Mist4_Level")]
    SgvAbilityMist4Level,
    #[serde(rename = "SGV_ABILITY_Mist_Level")]
    SgvAbilityMistLevel,
}
