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

pub type LevelTagExcelConfigData = Vec<LevelTagExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LevelTagExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "CCKBLEPEKCH")]
    pub cckblepekch: String,

    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "COBNIBENKIC")]
    pub cobnibenkic: Vec<i64>,

    #[serde(rename = "HHBHPBLABMF")]
    pub hhbhpblabmf: Vec<i64>,

    #[serde(rename = "AMHDPIFKPMC")]
    pub amhdpifkpmc: Option<bool>,

    #[serde(rename = "NFOCCDHDNPF")]
    pub nfoccdhdnpf: Option<f64>,

    #[serde(rename = "PGGMOAMLHAP")]
    pub pggmoamlhap: Vec<i64>,
}
