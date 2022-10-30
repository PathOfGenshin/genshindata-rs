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

pub type RogueMonsterPoolExcelConfigData = Vec<RogueMonsterPoolExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RogueMonsterPoolExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "OPBKGOJOPPG")]
    pub opbkgojoppg: Vec<i64>,

    #[serde(rename = "difficulty")]
    pub difficulty: Option<Difficulty>,
}

#[derive(Serialize, Deserialize)]
pub enum Difficulty {
    #[serde(rename = "ROGUE_MONSTER_DIFFICULTY_ELITE_EASY")]
    RogueMonsterDifficultyEliteEasy,

    #[serde(rename = "ROGUE_MONSTER_DIFFICULTY_ELITE_HARD")]
    RogueMonsterDifficultyEliteHard,
}
