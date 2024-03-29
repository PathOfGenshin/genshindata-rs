/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MainQuestExcelConfigData = Vec<MainQuestExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainQuestExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "type")]
    pub main_quest_excel_config_datum_type: Option<Type>,
    pub active_mode: Option<ActiveMode>,
    pub title_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub lua_path: String,
    pub suggest_track_main_quest_list: Vec<i64>,
    pub reward_id_list: Vec<i64>,
    pub show_type: Option<ShowType>,
    pub special_show_reward_id: Vec<i64>,
    pub special_show_cond_id_list: Vec<i64>,
    #[serde(rename = "OENJEOJBJBO")]
    pub oenjeojbjbo: Vec<i64>,
    pub repeatable: Option<bool>,
    pub res_id: Option<i64>,
    pub series: Option<i64>,
    pub chapter_id: Option<i64>,
    pub show_red_point: Option<bool>,
    #[serde(rename = "taskID")]
    pub task_id: Option<i64>,
    pub main_quest_tag: Option<MainQuestTag>,
    pub special_show_quest_id: Option<i64>,
    #[serde(rename = "NKMGAAOIGIL")]
    pub nkmgaaoigil: Option<bool>,
    pub suggest_track_out_of_order: Option<bool>,
    pub activity_id: Option<i64>,
    pub recommend_level: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActiveMode {
    #[serde(rename = "PLAY_MODE_SINGLE")]
    PlayModeSingle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MainQuestTag {
    #[serde(rename = "MAINQUEST_TAG_ACTIVITY")]
    MainquestTagActivity,
    #[serde(rename = "MAINQUEST_TAG_GUIDE")]
    MainquestTagGuide,
    #[serde(rename = "MAINQUEST_TAG_RANK_ZERO_WQ")]
    MainquestTagRankZeroWq,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShowType {
    #[serde(rename = "QUEST_HIDDEN")]
    QuestHidden,
}
