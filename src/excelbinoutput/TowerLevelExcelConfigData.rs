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

pub type TowerLevelExcelConfigData = Vec<TowerLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TowerLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "levelGroupId")]
    pub level_group_id: i64,

    #[serde(rename = "levelIndex")]
    pub level_index: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "conds")]
    pub conds: Vec<Cond>,

    #[serde(rename = "towerBuffConfigStrList")]
    pub tower_buff_config_str_list: Vec<String>,

    #[serde(rename = "firstPassRewardId")]
    pub first_pass_reward_id: i64,

    #[serde(rename = "monsterLevel")]
    pub monster_level: i64,

    #[serde(rename = "firstMonsterList")]
    pub first_monster_list: Vec<i64>,

    #[serde(rename = "secondMonsterList")]
    pub second_monster_list: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Cond {
    #[serde(rename = "towerCondType")]
    pub tower_cond_type: TowerCondType,

    #[serde(rename = "OECDJKGBNFE")]
    pub oecdjkgbnfe: Vec<i64>,

    #[serde(rename = "argumentList")]
    pub argument_list: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum TowerCondType {
    #[serde(rename = "TOWER_COND_CHALLENGE_LEFT_TIME_MORE_THAN")]
    TowerCondChallengeLeftTimeMoreThan,

    #[serde(rename = "TOWER_COND_LEFT_HP_GREATER_THAN")]
    TowerCondLeftHpGreaterThan,
}
