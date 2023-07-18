/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ReputationRequestExcelConfigData = Vec<ReputationRequestExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReputationRequestExcelConfigDatum {
    #[serde(rename = "RequestId")]
    pub request_id: i64,
    #[serde(rename = "QuestId")]
    pub quest_id: i64,
    #[serde(rename = "GroupId")]
    pub group_id: i64,
    pub weight: i64,
    pub npc_id: i64,
    pub reward_id: i64,
    pub name_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub icon_name: String,
}
