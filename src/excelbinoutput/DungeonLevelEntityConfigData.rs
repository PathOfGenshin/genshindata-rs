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

pub type DungeonLevelEntityConfigData = Vec<DungeonLevelEntityConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DungeonLevelEntityConfigDatum {
    #[serde(rename = "clientId")]
    pub client_id: i64,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "show")]
    pub show: Option<bool>,

    #[serde(rename = "levelConfigName")]
    pub level_config_name: String,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "LLNDCPOAMNP")]
    pub llndcpoamnp: i64,
}
