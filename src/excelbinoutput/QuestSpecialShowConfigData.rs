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

pub type QuestSpecialShowConfigData = Vec<QuestSpecialShowConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct QuestSpecialShowConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "condType")]
    pub cond_type: CondType,

    #[serde(rename = "param1")]
    pub param1: i64,

    #[serde(rename = "PBGCIBHOCFA")]
    pub pbgcibhocfa: i64,

    #[serde(rename = "param2")]
    pub param2: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum CondType {
    #[serde(rename = "SPECIAL_SHOW_FINISH")]
    SpecialShowFinish,

    #[serde(rename = "SPECIAL_SHOW_GLOBAL_VALUE_EQUAL_TO")]
    SpecialShowGlobalValueEqualTo,
}
