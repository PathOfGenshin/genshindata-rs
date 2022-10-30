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

pub type BoredMonsterPoolConfigData = Vec<BoredMonsterPoolConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BoredMonsterPoolConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "monsterId")]
    pub monster_id: i64,

    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "dropTag")]
    pub drop_tag: DropTag,

    #[serde(rename = "affixVec")]
    pub affix_vec: Vec<Option<serde_json::Value>>,

    #[serde(rename = "isElite")]
    pub is_elite: bool,
}

#[derive(Serialize, Deserialize)]
pub enum DropTag {
    #[serde(rename = "深渊法师")]
    DropTag,

    #[serde(rename = "丘丘暴徒")]
    Empty,

    #[serde(rename = "召唤师")]
    Fluffy,

    #[serde(rename = "元素之核")]
    Purple,

    #[serde(rename = "债务处理人")]
    Tentacled,
}
