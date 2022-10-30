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

pub type RoguelikeCursePoolExcelConfigData = Vec<RoguelikeCursePoolExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RoguelikeCursePoolExcelConfigDatum {
    #[serde(rename = "JMBECIIBMHM")]
    pub jmbeciibmhm: i64,

    #[serde(rename = "poolId")]
    pub pool_id: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,
}
