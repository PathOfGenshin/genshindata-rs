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

pub type DailyTaskExcelConfigData = Vec<DailyTaskExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DailyTaskExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "poolId")]
    pub pool_id: i64,

    #[serde(rename = "type")]
    pub daily_task_excel_config_datum_type: Option<Type>,

    #[serde(rename = "rarity")]
    pub rarity: Option<i64>,

    #[serde(rename = "oldGroupVec")]
    pub old_group_vec: Vec<i64>,

    #[serde(rename = "newGroupVec")]
    pub new_group_vec: Vec<i64>,

    #[serde(rename = "finishType")]
    pub finish_type: Option<FinishType>,

    #[serde(rename = "finishProgress")]
    pub finish_progress: Option<i64>,

    #[serde(rename = "taskRewardId")]
    pub task_reward_id: i64,

    #[serde(rename = "centerPosition")]
    pub center_position: String,

    #[serde(rename = "enterDistance")]
    pub enter_distance: i64,

    #[serde(rename = "exitDistance")]
    pub exit_distance: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descriptionTextMapHash")]
    pub description_text_map_hash: i64,

    #[serde(rename = "targetTextMapHash")]
    pub target_text_map_hash: i64,

    #[serde(rename = "radarRadius")]
    pub radar_radius: Option<f64>,

    #[serde(rename = "finishParam1")]
    pub finish_param1: Option<i64>,

    #[serde(rename = "finishParam2")]
    pub finish_param2: Option<i64>,

    #[serde(rename = "questId")]
    pub quest_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "DAILY_TASK_SCENE")]
    DailyTaskScene,
}

#[derive(Serialize, Deserialize)]
pub enum FinishType {
    #[serde(rename = "DAILY_FINISH_CHALLENGE")]
    DailyFinishChallenge,

    #[serde(rename = "DAILY_FINISH_GADGET_ID_NUM")]
    DailyFinishGadgetIdNum,

    #[serde(rename = "DAILY_FINISH_MONSTER_NUM")]
    DailyFinishMonsterNum,
}
