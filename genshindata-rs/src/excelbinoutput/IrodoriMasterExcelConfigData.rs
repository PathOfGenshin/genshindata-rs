/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type IrodoriMasterExcelConfigData = Vec<IrodoriMasterExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IrodoriMasterExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "levelID")]
    pub level_id: i64,
    pub level_type: LevelType,
    pub unlock_day: i64,
    pub watcher_list: Vec<i64>,
    #[serde(rename = "sliverChallengeID")]
    pub sliver_challenge_id: i64,
    #[serde(rename = "goldChallengeID")]
    pub gold_challenge_id: i64,
    #[serde(rename = "condID")]
    pub cond_id: i64,
    #[serde(rename = "guideCondID")]
    pub guide_cond_id: i64,
    #[serde(rename = "guideQuestID")]
    pub guide_quest_id: i64,
    pub battle_desc_text_map_hash: i64,
    pub battle_name_text_map_hash: i64,
    pub gold_challenge_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LevelType {
    #[serde(rename = "IRODORI_MASTER_LEVEL_HARD")]
    IrodoriMasterLevelHard,
    #[serde(rename = "IRODORI_MASTER_LEVEL_MASTER")]
    IrodoriMasterLevelMaster,
    #[serde(rename = "IRODORI_MASTER_LEVEL_NORAML")]
    IrodoriMasterLevelNoraml,
}
