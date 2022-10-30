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

pub type ClientSceneTagConfigData = Vec<ClientSceneTagConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ClientSceneTagConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "sceneTagName")]
    pub scene_tag_name: String,

    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "NECJFGPMKHG")]
    pub necjfgpmkhg: String,
}
