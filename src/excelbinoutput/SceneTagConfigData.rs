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

pub type SceneTagConfigData = Vec<SceneTagConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct SceneTagConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "sceneTagName")]
    pub scene_tag_name: String,

    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "GAFKBGEFGNE")]
    pub gafkbgefgne: Option<bool>,

    #[serde(rename = "cond")]
    pub cond: Vec<Cond>,

    #[serde(rename = "isDefaultValid")]
    pub is_default_valid: Option<bool>,

    #[serde(rename = "MDFOKFJALDH")]
    pub mdfokfjaldh: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Cond {
    #[serde(rename = "condType")]
    pub cond_type: Option<String>,

    #[serde(rename = "param1")]
    pub param1: Option<i64>,

    #[serde(rename = "param2")]
    pub param2: Option<i64>,
}
