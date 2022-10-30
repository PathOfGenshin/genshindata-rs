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

pub type MusicRiddlePlayConfigData = Vec<MusicRiddlePlayConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MusicRiddlePlayConfigDatum {
    #[serde(rename = "playId")]
    pub play_id: i64,

    #[serde(rename = "materialId")]
    pub material_id: i64,

    #[serde(rename = "HLGGMIPGGHF")]
    pub hlggmipgghf: String,

    #[serde(rename = "BIPLLOAMLFM")]
    pub biplloamlfm: f64,

    #[serde(rename = "HNAHEAKIAIB")]
    pub hnaheakiaib: Vec<i64>,

    #[serde(rename = "DKOBICCAIAL")]
    pub dkobiccaial: i64,
}
