/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type RandomMainQuestExcelConfigData = Vec<RandomMainQuestExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RandomMainQuestExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "type")]
    pub random_main_quest_excel_config_datum_type: Type,
    pub title_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub lua_path: String,
    pub active_mode: Option<String>,
    pub suggest_track_main_quest_list: Vec<Option<serde_json::Value>>,
    pub reward_id_list: String,
    pub hide_progress_sub_id_set_list: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "WQ")]
    Wq,
}
