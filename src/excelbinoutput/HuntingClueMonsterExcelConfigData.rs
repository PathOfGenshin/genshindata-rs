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

pub type HuntingClueMonsterExcelConfigData = Vec<HuntingClueMonsterExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HuntingClueMonsterExcelConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "monsterId")]
    pub monster_id: i64,

    #[serde(rename = "reviseLevelId")]
    pub revise_level_id: i64,

    #[serde(rename = "groupType")]
    pub group_type: GroupType,

    #[serde(rename = "monsterGroupId")]
    pub monster_group_id: i64,

    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "isClueMonster")]
    pub is_clue_monster: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum GroupType {
    #[serde(rename = "HUNTING_GROUP_TYPE_CLUE")]
    HuntingGroupTypeClue,
}
