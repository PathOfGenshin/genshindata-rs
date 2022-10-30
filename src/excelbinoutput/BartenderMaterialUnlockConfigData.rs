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

pub type BartenderMaterialUnlockConfigData = Vec<BartenderMaterialUnlockConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BartenderMaterialUnlockConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "HAFOEGJLOAK")]
    pub hafoegjloak: i64,

    #[serde(rename = "weight")]
    pub weight: i64,

    #[serde(rename = "HIJOHHJLLPB")]
    pub hijohhjllpb: i64,
}
