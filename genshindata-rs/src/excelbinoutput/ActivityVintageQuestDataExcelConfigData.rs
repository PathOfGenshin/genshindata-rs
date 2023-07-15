/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityVintageQuestDataExcelConfigData = Vec<ActivityVintageQuestDataExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityVintageQuestDataExcelConfigDatum {
    pub id: i64,
    pub quest_id: i64,
    #[serde(rename = "JMODNHKOEHJ")]
    pub jmodnhkoehj: i64,
    pub chapter_title_text_map_hash: i64,
    #[serde(rename = "EEEAIOEAEDC")]
    pub eeeaioeaedc: i64,
    #[serde(rename = "IJCEHGBANBF")]
    pub ijcehgbanbf: i64,
    pub pre_quest_id: Option<i64>,
}
