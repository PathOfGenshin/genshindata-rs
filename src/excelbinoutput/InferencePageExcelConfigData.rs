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

pub type InferencePageExcelConfigData = Vec<InferencePageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct InferencePageExcelConfigDatum {
    #[serde(rename = "DBLKEGDILKA")]
    pub dblkegdilka: i64,

    #[serde(rename = "parentQuestId")]
    pub parent_quest_id: i64,

    #[serde(rename = "DOPBHOCIDME")]
    pub dopbhocidme: Vec<i64>,

    #[serde(rename = "MINKOMACLFC")]
    pub minkomaclfc: i64,
}
