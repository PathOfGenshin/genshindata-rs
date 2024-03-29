/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type RogueDiaryQuestExcelConfigData = Vec<RogueDiaryQuestExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RogueDiaryQuestExcelConfigDatum {
    pub activity_id: i64,
    pub quest_id_list: Vec<String>,
    pub cond_id_list: Vec<i64>,
    pub watcher_id_list: Vec<i64>,
}
