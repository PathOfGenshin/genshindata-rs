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

pub type RogueDiaryResourceExcelConfigData = Vec<RogueDiaryResourceExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RogueDiaryResourceExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "type")]
    pub rogue_diary_resource_excel_config_datum_type: String,

    #[serde(rename = "value")]
    pub value: i64,
}
