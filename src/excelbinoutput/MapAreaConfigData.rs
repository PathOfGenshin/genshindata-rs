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

pub type MapAreaConfigData = Vec<MapAreaConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MapAreaConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "sceneID")]
    pub scene_id: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "GNKBBDPHMAG")]
    pub gnkbbdphmag: Vec<i64>,

    #[serde(rename = "GHEMDFCEDPE")]
    pub ghemdfcedpe: Option<i64>,

    #[serde(rename = "OOMELOMEEIA")]
    pub oomelomeeia: Option<Oomelomeeia>,
}

#[derive(Serialize, Deserialize)]
pub enum Oomelomeeia {
    #[serde(rename = "MistOpen")]
    MistOpen,
}
