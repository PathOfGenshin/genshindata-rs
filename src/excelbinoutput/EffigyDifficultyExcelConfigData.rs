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

pub type EffigyDifficultyExcelConfigData = Vec<EffigyDifficultyExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct EffigyDifficultyExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "challengeId")]
    pub challenge_id: i64,

    #[serde(rename = "baseScore")]
    pub base_score: i64,

    #[serde(rename = "monsterDifficulty")]
    pub monster_difficulty: MonsterDifficulty,

    #[serde(rename = "monsterLevel")]
    pub monster_level: i64,

    #[serde(rename = "scoreRatio")]
    pub score_ratio: f64,

    #[serde(rename = "AMOFABJCALM")]
    pub amofabjcalm: i64,
}

#[derive(Serialize, Deserialize)]
pub enum MonsterDifficulty {
    #[serde(rename = "EFFIGY_DIFFICULTY_EXPERT")]
    EffigyDifficultyExpert,

    #[serde(rename = "EFFIGY_DIFFICULTY_HARD")]
    EffigyDifficultyHard,

    #[serde(rename = "EFFIGY_DIFFICULTY_NORMAL")]
    EffigyDifficultyNormal,

    #[serde(rename = "EFFIGY_DIFFICULTY_PRIMER")]
    EffigyDifficultyPrimer,
}
