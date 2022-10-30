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

pub type LevelTagGroupsExcelConfigData = Vec<LevelTagGroupsExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LevelTagGroupsExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "LNDNJDKLPCA")]
    pub lndnjdklpca: Vec<Lndnjdklpca>,

    #[serde(rename = "DPIHKLDDOGC")]
    pub dpihklddogc: Vec<i64>,

    #[serde(rename = "DNEOKPJIAMI")]
    pub dneokpjiami: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Lndnjdklpca {
    #[serde(rename = "JCAMMOFCBON")]
    pub jcammofcbon: Vec<i64>,
}
