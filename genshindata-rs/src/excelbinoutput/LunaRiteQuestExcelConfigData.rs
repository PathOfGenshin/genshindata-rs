/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LunaRiteQuestExcelConfigData = Vec<LunaRiteQuestExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LunaRiteQuestExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,
    pub quest_id: i64,
    pub open_day: i64,
    pub chapter_icon: String,
    #[serde(rename = "HDHDAMLEDBM")]
    pub hdhdamledbm: i64,
    pub name_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub pre_quest_id: Option<i64>,
    #[serde(rename = "LJEGIECPPAM")]
    pub ljegiecppam: Option<i64>,
    #[serde(rename = "BMBFLKCJBFI")]
    pub bmbflkcjbfi: Option<String>,
}
