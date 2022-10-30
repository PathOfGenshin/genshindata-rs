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

pub type AchievementExcelConfigData = Vec<AchievementExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AchievementExcelConfigDatum {
    #[serde(rename = "goalId")]
    pub goal_id: Option<i64>,

    #[serde(rename = "orderId")]
    pub order_id: Option<i64>,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "ps5TitleTextMapHash")]
    pub ps5_title_text_map_hash: i64,

    #[serde(rename = "ttype")]
    pub ttype: Ttype,

    #[serde(rename = "psTrophyId")]
    pub ps_trophy_id: String,

    #[serde(rename = "ps4TrophyId")]
    pub ps4_trophy_id: String,

    #[serde(rename = "ps5TrophyId")]
    pub ps5_trophy_id: String,

    #[serde(rename = "icon")]
    pub icon: Icon,

    #[serde(rename = "finishRewardId")]
    pub finish_reward_id: i64,

    #[serde(rename = "isDeleteWatcherAfterFinish")]
    pub is_delete_watcher_after_finish: Option<bool>,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "triggerConfig")]
    pub trigger_config: TriggerConfig,

    #[serde(rename = "progress")]
    pub progress: i64,

    #[serde(rename = "ps4GroupId")]
    pub ps4_group_id: Option<i64>,

    #[serde(rename = "ps5GroupId")]
    pub ps5_group_id: Option<i64>,

    #[serde(rename = "preStageAchievementId")]
    pub pre_stage_achievement_id: Option<i64>,

    #[serde(rename = "isShow")]
    pub is_show: Option<IsShow>,

    #[serde(rename = "isDisuse")]
    pub is_disuse: Option<bool>,

    #[serde(rename = "progressShowType")]
    pub progress_show_type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TriggerConfig {
    #[serde(rename = "triggerType")]
    pub trigger_type: String,

    #[serde(rename = "paramList")]
    pub param_list: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Icon {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_AchievementIcon_A001")]
    UiAchievementIconA001,

    #[serde(rename = "UI_AchievementIcon_A001_Part2")]
    UiAchievementIconA001Part2,

    #[serde(rename = "UI_AchievementIcon_A001_Part3")]
    UiAchievementIconA001Part3,

    #[serde(rename = "UI_AchievementIcon_A003")]
    UiAchievementIconA003,

    #[serde(rename = "UI_AchievementIcon_A004")]
    UiAchievementIconA004,

    #[serde(rename = "UI_AchievementIcon_A005")]
    UiAchievementIconA005,

    #[serde(rename = "UI_AchievementIcon_A006")]
    UiAchievementIconA006,

    #[serde(rename = "UI_AchievementIcon_A008")]
    UiAchievementIconA008,

    #[serde(rename = "UI_AchievementIcon_A012")]
    UiAchievementIconA012,

    #[serde(rename = "UI_AchievementIcon_A013")]
    UiAchievementIconA013,

    #[serde(rename = "UI_AchievementIcon_B002")]
    UiAchievementIconB002,

    #[serde(rename = "UI_AchievementIcon_B005")]
    UiAchievementIconB005,

    #[serde(rename = "UI_AchievementIcon_O001")]
    UiAchievementIconO001,
}

#[derive(Serialize, Deserialize)]
pub enum IsShow {
    #[serde(rename = "SHOWTYPE_HIDE")]
    ShowtypeHide,
}

#[derive(Serialize, Deserialize)]
pub enum Ttype {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "金")]
    Fluffy,

    #[serde(rename = "银")]
    Purple,

    #[serde(rename = "铜")]
    Ttype,
}