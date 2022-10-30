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

pub type BattlePassScheduleExcelConfigData = Vec<BattlePassScheduleExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BattlePassScheduleExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "titleNameTextMapHash")]
    pub title_name_text_map_hash: i64,

    #[serde(rename = "beginDateStr")]
    pub begin_date_str: String,

    #[serde(rename = "endDateStr")]
    pub end_date_str: String,

    #[serde(rename = "cycleList")]
    pub cycle_list: Vec<i64>,

    #[serde(rename = "extraPaidRewardId")]
    pub extra_paid_reward_id: i64,

    #[serde(rename = "extraPaidAddPoint")]
    pub extra_paid_add_point: i64,

    #[serde(rename = "buyLevelCostCoinNum")]
    pub buy_level_cost_coin_num: i64,

    #[serde(rename = "cyclePointUpperLimit")]
    pub cycle_point_upper_limit: i64,

    #[serde(rename = "levelRewardIndexId")]
    pub level_reward_index_id: i64,

    #[serde(rename = "normalRewardList")]
    pub normal_reward_list: Vec<i64>,

    #[serde(rename = "showImage")]
    pub show_image: ShowImage,

    #[serde(rename = "showRewardList")]
    pub show_reward_list: Vec<i64>,

    #[serde(rename = "stroyRewardList")]
    pub stroy_reward_list: Vec<i64>,

    #[serde(rename = "storyId")]
    pub story_id: i64,

    #[serde(rename = "mailDayCount")]
    pub mail_day_count: Option<i64>,

    #[serde(rename = "mailConfigId")]
    pub mail_config_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum ShowImage {
    #[serde(rename = "UI_BattlePass_Reward_01")]
    UiBattlePassReward01,
}
