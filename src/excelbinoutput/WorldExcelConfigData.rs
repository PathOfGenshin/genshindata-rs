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

pub type WorldExcelConfigData = Vec<WorldExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WorldExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "type")]
    pub world_excel_config_datum_type: String,

    #[serde(rename = "mainSceneId")]
    pub main_scene_id: i64,

    #[serde(rename = "subSceneIdVec")]
    pub sub_scene_id_vec: Vec<i64>,
}
