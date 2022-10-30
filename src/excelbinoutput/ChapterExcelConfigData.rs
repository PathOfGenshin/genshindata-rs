// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

pub type ChapterExcelConfigData = Vec<ChapterExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ChapterExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "beginQuestId")]
    pub begin_quest_id: Option<i64>,

    #[serde(rename = "endQuestId")]
    pub end_quest_id: Option<i64>,

    #[serde(rename = "needBeginTimeStr")]
    pub need_begin_time_str: String,

    #[serde(rename = "chapterNumTextMapHash")]
    pub chapter_num_text_map_hash: i64,

    #[serde(rename = "chapterTitleTextMapHash")]
    pub chapter_title_text_map_hash: i64,

    #[serde(rename = "chapterIcon")]
    pub chapter_icon: String,

    #[serde(rename = "chapterImageHashSuffix")]
    pub chapter_image_hash_suffix: Option<i64>,

    #[serde(rename = "chapterImageHashPre")]
    pub chapter_image_hash_pre: Option<i64>,

    #[serde(rename = "chapterImageTitleTextMapHash")]
    pub chapter_image_title_text_map_hash: i64,

    #[serde(rename = "chapterSerialNumberIcon")]
    pub chapter_serial_number_icon: ChapterSerialNumberIcon,

    #[serde(rename = "needPlayerLevel")]
    pub need_player_level: Option<i64>,

    #[serde(rename = "activityId")]
    pub activity_id: Option<i64>,

    #[serde(rename = "inActivityNeedPlayerLevel")]
    pub in_activity_need_player_level: Option<i64>,
}

#[derive(Serialize, Deserialize)]
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
}
