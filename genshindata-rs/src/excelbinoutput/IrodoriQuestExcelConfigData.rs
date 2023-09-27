/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type IrodoriQuestExcelConfigData = Vec<IrodoriQuestExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IrodoriQuestExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,
    pub quest_id: i64,
    pub main_quest_ids: Vec<i64>,
    pub open_day: i64,
    pub chapter_text_map_hash: i64,
    pub name_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub pre_other_quest_ids: Vec<Option<serde_json::Value>>,
    pub pre_quest_id: Option<i64>,
    pub quest_type: Option<String>,
}
