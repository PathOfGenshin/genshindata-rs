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

pub type DieTypeTipsExcelConfigData = Vec<DieTypeTipsExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DieTypeTipsExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "dieType")]
    pub die_type: DieType,

    #[serde(rename = "tips")]
    pub tips: Vec<i64>,

    #[serde(rename = "monsterID")]
    pub monster_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum DieType {
    #[serde(rename = "PLAYER_DIE_ABYSS")]
    PlayerDieAbyss,

    #[serde(rename = "PLAYER_DIE_DRAWN")]
    PlayerDieDrawn,

    #[serde(rename = "PLAYER_DIE_FALL")]
    PlayerDieFall,

    #[serde(rename = "PLAYER_DIE_KILL_BY_MONSTER")]
    PlayerDieKillByMonster,
}
