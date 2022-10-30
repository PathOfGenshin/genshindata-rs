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

pub type CustomLevelComponentTypeConfigData = Vec<CustomLevelComponentTypeConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CustomLevelComponentTypeConfigDatum {
    #[serde(rename = "typeID")]
    pub type_id: i64,

    #[serde(rename = "typeNameTextMapHash")]
    pub type_name_text_map_hash: i64,
}
