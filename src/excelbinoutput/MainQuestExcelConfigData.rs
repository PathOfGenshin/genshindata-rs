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

pub type MainQuestExcelConfigData = Vec<MainQuestExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MainQuestExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "type")]
    pub main_quest_excel_config_datum_type: Option<Type>,

    #[serde(rename = "activeMode")]
    pub active_mode: Option<ActiveMode>,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "luaPath")]
    pub lua_path: String,

    #[serde(rename = "suggestTrackMainQuestList")]
    pub suggest_track_main_quest_list: Vec<i64>,

    #[serde(rename = "rewardIdList")]
    pub reward_id_list: Vec<i64>,

    #[serde(rename = "showType")]
    pub show_type: Option<ShowType>,

    #[serde(rename = "GNDEJDHMDKC")]
    pub gndejdhmdkc: Vec<i64>,

    #[serde(rename = "NDBCKBLAKJJ")]
    pub ndbckblakjj: Vec<i64>,

    #[serde(rename = "repeatable")]
    pub repeatable: Option<bool>,

    #[serde(rename = "NOJGFFLGHAB")]
    pub nojgfflghab: Option<i64>,

    #[serde(rename = "series")]
    pub series: Option<i64>,

    #[serde(rename = "chapterId")]
    pub chapter_id: Option<i64>,

    #[serde(rename = "showRedPoint")]
    pub show_red_point: Option<bool>,

    #[serde(rename = "taskID")]
    pub task_id: Option<i64>,

    #[serde(rename = "mainQuestTag")]
    pub main_quest_tag: Option<MainQuestTag>,

    #[serde(rename = "DKMAJDKOJLM")]
    pub dkmajdkojlm: Option<i64>,

    #[serde(rename = "suggestTrackOutOfOrder")]
    pub suggest_track_out_of_order: Option<bool>,

    #[serde(rename = "activityId")]
    pub activity_id: Option<i64>,

    #[serde(rename = "recommendLevel")]
    pub recommend_level: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum ActiveMode {
    #[serde(rename = "PLAY_MODE_SINGLE")]
    PlayModeSingle,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "EQ")]
    Eq,

    #[serde(rename = "IQ")]
    Iq,

    #[serde(rename = "LQ")]
    Lq,

    #[serde(rename = "WQ")]
    Wq,
}

#[derive(Serialize, Deserialize)]
pub enum MainQuestTag {
    #[serde(rename = "MAINQUEST_TAG_ACTIVITY")]
    MainquestTagActivity,

    #[serde(rename = "MAINQUEST_TAG_GUIDE")]
    MainquestTagGuide,

    #[serde(rename = "MAINQUEST_TAG_MAIN_WQ")]
    MainquestTagMainWq,

    #[serde(rename = "MAINQUEST_TAG_RANK_ZERO_WQ")]
    MainquestTagRankZeroWq,
}

#[derive(Serialize, Deserialize)]
pub enum ShowType {
    #[serde(rename = "QUEST_HIDDEN")]
    QuestHidden,
}
