/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BlossomRefreshExcelConfigData = Vec<BlossomRefreshExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlossomRefreshExcelConfigDatum {
    pub id: i64,
    pub name_text_map_hash: i64,
    pub icon: String,
    pub desc_text_map_hash: i64,
    pub city_id: i64,
    pub refresh_type: String,
    pub refresh_count: i64,
    pub refresh_time: String,
    pub refresh_cond_vec: Vec<RefreshCondVec>,
    pub revise_level: i64,
    pub blossom_chest_id: Option<i64>,
    pub camp_update_need_count: Option<i64>,
    pub drop_vec: Vec<DropVec>,
    pub client_show_type: ClientShowType,
    pub item_limit_type: Option<String>,
    pub reward_type: Option<String>,
    #[serde(rename = "PAIOGEFEABG")]
    pub paiogefeabg: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientShowType {
    #[serde(rename = "BLOSSOM_SHOWTYPE_CHALLENGE")]
    BlossomShowtypeChallenge,
    #[serde(rename = "BLOSSOM_SHOWTYPE_GROUPCHALLENGE")]
    BlossomShowtypeGroupchallenge,
    #[serde(rename = "BLOSSOM_SHOWTYPE_NPCTALK")]
    BlossomShowtypeNpctalk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DropVec {
    pub drop_id: i64,
    pub preview_reward: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshCondVec {
    #[serde(rename = "type")]
    pub refresh_cond_vec_type: Option<Type>,
    pub param: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    #[serde(rename = "BLOSSOM_REFRESH_COND_ACTIVITY_COND")]
    BlossomRefreshCondActivityCond,
    #[serde(rename = "BLOSSOM_REFRESH_COND_OPEN_STATE")]
    BlossomRefreshCondOpenState,
    #[serde(rename = "BLOSSOM_REFRESH_COND_PLAYER_LEVEL_EQUAL_GREATER")]
    BlossomRefreshCondPlayerLevelEqualGreater,
    #[serde(rename = "BLOSSOM_REFRESH_COND_PLAYER_LEVEL_LESS_THAN")]
    BlossomRefreshCondPlayerLevelLessThan,
    #[serde(rename = "BLOSSOM_REFRESH_COND_SCENE_TAG_ADDED")]
    BlossomRefreshCondSceneTagAdded,
}
