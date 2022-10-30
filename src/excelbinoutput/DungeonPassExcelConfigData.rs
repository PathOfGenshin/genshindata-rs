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

pub type DungeonPassExcelConfigData = Vec<DungeonPassExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DungeonPassExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "conds")]
    pub conds: Vec<Cond>,

    #[serde(rename = "logicType")]
    pub logic_type: Option<LogicType>,
}

#[derive(Serialize, Deserialize)]
pub struct Cond {
    #[serde(rename = "condType")]
    pub cond_type: Option<CondType>,

    #[serde(rename = "param")]
    pub param: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum CondType {
    #[serde(rename = "DUNGEON_COND_END_MULTISTAGE_PLAY")]
    DungeonCondEndMultistagePlay,

    #[serde(rename = "DUNGEON_COND_FINISH_CHALLENGE")]
    DungeonCondFinishChallenge,

    #[serde(rename = "DUNGEON_COND_FINISH_QUEST")]
    DungeonCondFinishQuest,

    #[serde(rename = "DUNGEON_COND_IN_TIME")]
    DungeonCondInTime,

    #[serde(rename = "DUNGEON_COND_KILL_GROUP_MONSTER")]
    DungeonCondKillGroupMonster,

    #[serde(rename = "DUNGEON_COND_KILL_MONSTER")]
    DungeonCondKillMonster,

    #[serde(rename = "DUNGEON_COND_KILL_MONSTER_COUNT")]
    DungeonCondKillMonsterCount,

    #[serde(rename = "DUNGEON_COND_KILL_TYPE_MONSTER")]
    DungeonCondKillTypeMonster,
}

#[derive(Serialize, Deserialize)]
pub enum LogicType {
    #[serde(rename = "LOGIC_AND")]
    LogicAnd,

    #[serde(rename = "LOGIC_OR")]
    LogicOr,
}
