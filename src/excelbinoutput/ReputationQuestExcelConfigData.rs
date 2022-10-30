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

pub type ReputationQuestExcelConfigData = Vec<ReputationQuestExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ReputationQuestExcelConfigDatum {
    #[serde(rename = "ParentQuestId")]
    pub parent_quest_id: i64,

    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "iconName")]
    pub icon_name: IconName,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "order")]
    pub order: i64,
}

#[derive(Serialize, Deserialize)]
pub enum IconName {
    #[serde(rename = "UI_ChapterIcon_Inazuma")]
    UiChapterIconInazuma,

    #[serde(rename = "UI_ChapterIcon_Liyue")]
    UiChapterIconLiyue,

    #[serde(rename = "UI_ChapterIcon_Mengde")]
    UiChapterIconMengde,

    #[serde(rename = "UI_ChapterIcon_Sumeru")]
    UiChapterIconSumeru,

    #[serde(rename = "UI_QuestIcon_Quest")]
    UiQuestIconQuest,
}
