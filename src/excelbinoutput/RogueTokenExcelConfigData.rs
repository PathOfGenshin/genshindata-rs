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

pub type RogueTokenExcelConfigData = Vec<RogueTokenExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RogueTokenExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "PNONIJHOHOA")]
    pub pnonijhohoa: Vec<i64>,

    #[serde(rename = "DIBNIILMPHD")]
    pub dibniilmphd: Vec<i64>,

    #[serde(rename = "CFBLKKMKIMH")]
    pub cfblkkmkimh: Vec<i64>,

    #[serde(rename = "FHDNMLEJGIB")]
    pub fhdnmlejgib: Option<Fhdnmlejgib>,
}

#[derive(Serialize, Deserialize)]
pub enum Fhdnmlejgib {
    #[serde(rename = "ROGUE_MONSTER_DIFFICULTY_BOSS")]
    RogueMonsterDifficultyBoss,

    #[serde(rename = "ROGUE_MONSTER_DIFFICULTY_ELITE_EASY")]
    RogueMonsterDifficultyEliteEasy,

    #[serde(rename = "ROGUE_MONSTER_DIFFICULTY_ELITE_HARD")]
    RogueMonsterDifficultyEliteHard,
}
