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

pub type MaterialSourceDataExcelConfigData = Vec<MaterialSourceDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MaterialSourceDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "dungeonList")]
    pub dungeon_list: Vec<i64>,

    #[serde(rename = "GJMLDACGHFJ")]
    pub gjmldacghfj: Vec<Gjmldacghfj>,

    #[serde(rename = "JCLGCACJMMK")]
    pub jclgcacjmmk: Vec<i64>,

    #[serde(rename = "CDCOJJJOADI")]
    pub cdcojjjoadi: Vec<Cdcojjjoadi>,

    #[serde(rename = "KFPOMCBFKHM")]
    pub kfpomcbfkhm: Vec<i64>,

    #[serde(rename = "jumpList")]
    pub jump_list: Vec<i64>,

    #[serde(rename = "textList")]
    pub text_list: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Gjmldacghfj {}

#[derive(Serialize, Deserialize)]
pub enum Cdcojjjoadi {
    #[serde(rename = "JUMP_INVESTIGATION")]
    JumpInvestigation,

    #[serde(rename = "JUMP_MALL")]
    JumpMall,
}
