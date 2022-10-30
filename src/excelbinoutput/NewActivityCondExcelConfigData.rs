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

pub type NewActivityCondExcelConfigData = Vec<NewActivityCondExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct NewActivityCondExcelConfigDatum {
    #[serde(rename = "condId")]
    pub cond_id: i64,

    #[serde(rename = "condComb")]
    pub cond_comb: Option<CondComb>,

    #[serde(rename = "cond")]
    pub cond: Vec<Cond>,
}

#[derive(Serialize, Deserialize)]
pub struct Cond {
    #[serde(rename = "type")]
    pub cond_type: Option<String>,

    #[serde(rename = "param")]
    pub param: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum CondComb {
    #[serde(rename = "LOGIC_AND")]
    LogicAnd,

    #[serde(rename = "LOGIC_OR")]
    LogicOr,
}
