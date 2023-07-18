/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ChapterExcelConfigData = Vec<ChapterExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterExcelConfigDatum {
    pub id: i64,
    pub begin_quest_id: Option<i64>,
    pub end_quest_id: Option<i64>,
    pub need_begin_time_str: String,
    pub chapter_num_text_map_hash: i64,
    pub chapter_title_text_map_hash: i64,
    pub chapter_icon: String,
    #[serde(rename = "JHJMHNOOELP")]
    pub jhjmhnooelp: Option<f64>,
    pub chapter_image_title_text_map_hash: i64,
    pub chapter_serial_number_icon: ChapterSerialNumberIcon,
    pub need_player_level: Option<i64>,
    pub activity_id: Option<i64>,
    pub in_activity_need_player_level: Option<i64>,
    pub city_id: Option<i64>,
    pub chapter_image_hash: Option<i64>,
    #[serde(rename = "OGMNHMEOFBM")]
    pub ogmnhmeofbm: Option<Ogmnhmeofbm>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChapterSerialNumberIcon {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "UI_QuestType_Chapter_1")]
    UiQuestTypeChapter1,
    #[serde(rename = "UI_QuestType_Chapter_2")]
    UiQuestTypeChapter2,
    #[serde(rename = "UI_QuestType_Chapter_3")]
    UiQuestTypeChapter3,
    #[serde(rename = "UI_QuestType_Chapter_4")]
    UiQuestTypeChapter4,
    #[serde(rename = "UI_QuestType_Chapter_5")]
    UiQuestTypeChapter5,
    #[serde(rename = "UI_QuestType_Chapter_6")]
    UiQuestTypeChapter6,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Ogmnhmeofbm {
    #[serde(rename = "CHAPTER_STYLE_TYPE_WORLD_QUEST_RANK_ZERO")]
    ChapterStyleTypeWorldQuestRankZero,
}
