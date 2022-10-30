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

pub type MonsterRelOverloadExcelConfigData = Vec<MonsterRelOverloadExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MonsterRelOverloadExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "BHJFOEBAOAJ")]
    pub bhjfoebaoaj: Bhjfoebaoaj,

    #[serde(rename = "BFFGBGKFKPH")]
    pub bffgbgkfkph: Bffgbgkfkph,

    #[serde(rename = "LOPAABGELAM")]
    pub lopaabgelam: Vec<Lopaabgelam>,
}

#[derive(Serialize, Deserialize)]
pub struct Lopaabgelam {
    #[serde(rename = "paramList")]
    pub param_list: Vec<i64>,

    #[serde(rename = "DCGJDEABNHO")]
    pub dcgjdeabnho: Dcgjdeabnho,

    #[serde(rename = "monsterRarity")]
    pub monster_rarity: MonsterRarity,
}

#[derive(Serialize, Deserialize)]
pub enum Bffgbgkfkph {
    #[serde(rename = "_MONSTER_FUNGUSSTATE_")]
    MonsterFungusstate,
}

#[derive(Serialize, Deserialize)]
pub enum Bhjfoebaoaj {
    #[serde(rename = "MONSTER_POLY_DROP_GV")]
    MonsterPolyDropGv,
}

#[derive(Serialize, Deserialize)]
pub enum Dcgjdeabnho {
    #[serde(rename = "异化蕈兽")]
    Dcgjdeabnho,

    #[serde(rename = "蕈兽")]
    Empty,

    #[serde(rename = "大异化蕈兽")]
    Fluffy,

    #[serde(rename = "大蕈兽")]
    Purple,
}

#[derive(Serialize, Deserialize)]
pub enum MonsterRarity {
    #[serde(rename = "MONSTER_RARITY_ELITE_MONSTER")]
    MonsterRarityEliteMonster,

    #[serde(rename = "MONSTER_RARITY_SMALL_MONSTER")]
    MonsterRaritySmallMonster,
}
