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

pub type FleurFairDungeonExcelConfigData = Vec<FleurFairDungeonExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FleurFairDungeonExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "sectionId")]
    pub section_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "obtainCoinLimit")]
    pub obtain_coin_limit: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "miniGameList")]
    pub mini_game_list: Vec<i64>,

    #[serde(rename = "monsterList")]
    pub monster_list: Vec<i64>,

    #[serde(rename = "energyGradeList")]
    pub energy_grade_list: Vec<i64>,

    #[serde(rename = "initialEnergy")]
    pub initial_energy: i64,

    #[serde(rename = "energyLimit")]
    pub energy_limit: i64,

    #[serde(rename = "successRewardId")]
    pub success_reward_id: i64,

    #[serde(rename = "failureRewardId")]
    pub failure_reward_id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
}
