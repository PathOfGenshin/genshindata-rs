/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type NewActivityMainQuestDataExcelConfigData = Vec<NewActivityMainQuestDataExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct NewActivityMainQuestDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub iencjoldppo: i64,
    #[serde(rename = "questIdList")]
    pub quest_id_list: Vec<i64>,
    pub jmodnhkoehj: i64,
    #[serde(rename = "chapterTitleTextMapHash")]
    pub chapter_title_text_map_hash: i64,
    pub eeeaioeaedc: i64,
    pub ijcehgbanbf: i64,
    #[serde(rename = "preQuestId")]
    pub pre_quest_id: Option<i64>,
    pub djpiddmphpo: Option<i64>,
}
