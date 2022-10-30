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

pub type MonsterRelationshipExcelConfigData = Vec<MonsterRelationshipExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MonsterRelationshipExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "tagStr")]
    pub tag_str: String,

    #[serde(rename = "monsterRarity")]
    pub monster_rarity: MonsterRarity,

    #[serde(rename = "BLGCIFIJPPI")]
    pub blgcifijppi: String,

    #[serde(rename = "JDPJINJEANM")]
    pub jdpjinjeanm: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum MonsterRarity {
    #[serde(rename = "MONSTER_RARITY_BIG_BOSS_MONSTER")]
    MonsterRarityBigBossMonster,

    #[serde(rename = "MONSTER_RARITY_BOSS_MONSTER")]
    MonsterRarityBossMonster,

    #[serde(rename = "MONSTER_RARITY_ELITE_MONSTER")]
    MonsterRarityEliteMonster,

    #[serde(rename = "MONSTER_RARITY_SMALL_ENV_ANIMAL")]
    MonsterRaritySmallEnvAnimal,

    #[serde(rename = "MONSTER_RARITY_SMALL_MONSTER")]
    MonsterRaritySmallMonster,
}
